#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: CaretType = CaretType(0i32);
    pub const Null: CaretType = CaretType(1i32);
}
impl ::std::convert::From<i32> for CaretType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CaretType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CaretType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.CaretType;i4)");
}
impl ::windows::runtime::DefaultType for CaretType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: FindOptions = FindOptions(0u32);
    pub const Word: FindOptions = FindOptions(2u32);
    pub const Case: FindOptions = FindOptions(4u32);
}
impl ::std::convert::From<u32> for FindOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FindOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FindOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.FindOptions;u4)");
}
impl ::windows::runtime::DefaultType for FindOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Text`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct FontWeights(pub ::windows::runtime::IInspectable);
impl FontWeights {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Black() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Bold() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ExtraBlack() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ExtraBold() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ExtraLight() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Light() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Medium() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Normal() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SemiBold() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SemiLight() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Thin() -> ::windows::runtime::Result<::windows::UI::Text::FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        })
    }
    pub fn IFontWeightsStatics<
        R,
        F: FnOnce(&IFontWeightsStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FontWeights, IFontWeightsStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FontWeights {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Text.FontWeights;{386cd040-5404-5a8d-8bc7-2ca989f5c065})",
    );
}
unsafe impl ::windows::runtime::Interface for FontWeights {
    type Vtable = IFontWeights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        946655296,
        21508,
        23181,
        [139, 199, 44, 169, 137, 245, 192, 101],
    );
}
impl ::windows::runtime::RuntimeName for FontWeights {
    const NAME: &'static str = "Microsoft.UI.Text.FontWeights";
}
impl ::std::convert::From<FontWeights> for ::windows::runtime::IUnknown {
    fn from(value: FontWeights) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&FontWeights> for ::windows::runtime::IUnknown {
    fn from(value: &FontWeights) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<FontWeights> for ::windows::runtime::IInspectable {
    fn from(value: FontWeights) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FontWeights> for ::windows::runtime::IInspectable {
    fn from(value: &FontWeights) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FontWeights {}
unsafe impl ::std::marker::Sync for FontWeights {}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: FormatEffect = FormatEffect(0i32);
    pub const On: FormatEffect = FormatEffect(1i32);
    pub const Toggle: FormatEffect = FormatEffect(2i32);
    pub const Undefined: FormatEffect = FormatEffect(3i32);
}
impl ::std::convert::From<i32> for FormatEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FormatEffect {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FormatEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.FormatEffect;i4)");
}
impl ::windows::runtime::DefaultType for FormatEffect {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: HorizontalCharacterAlignment = HorizontalCharacterAlignment(0i32);
    pub const Right: HorizontalCharacterAlignment = HorizontalCharacterAlignment(1i32);
    pub const Center: HorizontalCharacterAlignment = HorizontalCharacterAlignment(2i32);
}
impl ::std::convert::From<i32> for HorizontalCharacterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HorizontalCharacterAlignment {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HorizontalCharacterAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.HorizontalCharacterAlignment;i4)",
    );
}
impl ::windows::runtime::DefaultType for HorizontalCharacterAlignment {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFontWeights(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFontWeights {
    type Vtable = IFontWeights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        946655296,
        21508,
        23181,
        [139, 199, 44, 169, 137, 245, 192, 101],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeights_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFontWeightsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFontWeightsStatics {
    type Vtable = IFontWeightsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3426291190,
        30384,
        22535,
        [139, 157, 233, 73, 164, 230, 35, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeightsStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Text`*"]
pub struct ITextCharacterFormat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextCharacterFormat {
    type Vtable = ITextCharacterFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4117823568,
        39141,
        22408,
        [177, 227, 50, 25, 30, 235, 249, 77],
    );
}
impl ITextCharacterFormat {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn AllCaps(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetAllCaps(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Bold(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetBold(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FontStretch(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FontStyle(&self) -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetForegroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Hidden(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetHidden(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Italic(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetItalic(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Kerning(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetKerning(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LanguageTag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetLanguageTag<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LinkType(&self) -> ::windows::runtime::Result<LinkType> {
        let this = self;
        unsafe {
            let mut result__: LinkType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LinkType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Outline(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetOutline(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetPosition(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ProtectedText(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetProtectedText(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSize(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SmallCaps(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSmallCaps(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Spacing(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSpacing(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Strikethrough(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetStrikethrough(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Subscript(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSubscript(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Superscript(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSuperscript(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn TextScript(&self) -> ::windows::runtime::Result<TextScript> {
        let this = self;
        unsafe {
            let mut result__: TextScript = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<TextScript>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetTextScript(&self, value: TextScript) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Underline(&self) -> ::windows::runtime::Result<UnderlineType> {
        let this = self;
        unsafe {
            let mut result__: UnderlineType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UnderlineType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetUnderline(&self, value: UnderlineType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Weight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetWeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetClone<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(
        &self,
        format: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                format.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextCharacterFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{f5710050-98e5-5788-b1e3-32191eebf94d}");
}
impl ::std::convert::From<ITextCharacterFormat> for ::windows::runtime::IUnknown {
    fn from(value: ITextCharacterFormat) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ITextCharacterFormat> for ::windows::runtime::IUnknown {
    fn from(value: &ITextCharacterFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextCharacterFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ITextCharacterFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ITextCharacterFormat> for ::windows::runtime::IInspectable {
    fn from(value: ITextCharacterFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextCharacterFormat> for ::windows::runtime::IInspectable {
    fn from(value: &ITextCharacterFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITextCharacterFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextCharacterFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextCharacterFormat_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut LinkType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut TextScript,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: TextScript,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut UnderlineType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: UnderlineType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextConstantsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextConstantsStatics {
    type Vtable = ITextConstantsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3442817852,
        44899,
        23803,
        [145, 140, 15, 156, 137, 49, 161, 97],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConstantsStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextDocument(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextDocument {
    type Vtable = ITextDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        290051453,
        34470,
        23005,
        [136, 217, 25, 111, 39, 188, 92, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut CaretType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: CaretType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        startposition: i32,
        endposition: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        point: ::windows::Foundation::Point,
        options: PointOptions,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextGetOptions,
        value: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextSetOptions,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextGetOptions,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextSetOptions,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Text`*"]
pub struct ITextParagraphFormat(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextParagraphFormat {
    type Vtable = ITextParagraphFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        563834079,
        3339,
        22273,
        [184, 161, 108, 144, 107, 62, 187, 225],
    );
}
impl ITextParagraphFormat {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Alignment(&self) -> ::windows::runtime::Result<ParagraphAlignment> {
        let this = self;
        unsafe {
            let mut result__: ParagraphAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ParagraphAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FirstLineIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn KeepTogether(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetKeepTogether(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn KeepWithNext(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LeftIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LineSpacingRule(&self) -> ::windows::runtime::Result<LineSpacingRule> {
        let this = self;
        unsafe {
            let mut result__: LineSpacingRule = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<LineSpacingRule>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListAlignment(&self) -> ::windows::runtime::Result<MarkerAlignment> {
        let this = self;
        unsafe {
            let mut result__: MarkerAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MarkerAlignment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListLevelIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListLevelIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListStart(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListStart(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListStyle(&self) -> ::windows::runtime::Result<MarkerStyle> {
        let this = self;
        unsafe {
            let mut result__: MarkerStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MarkerStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListStyle(&self, value: MarkerStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListTab(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListTab(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ListType(&self) -> ::windows::runtime::Result<MarkerType> {
        let this = self;
        unsafe {
            let mut result__: MarkerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MarkerType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetListType(&self, value: MarkerType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn NoLineNumber(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn PageBreakBefore(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn RightIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetRightIndent(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn RightToLeft(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetRightToLeft(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Style(&self) -> ::windows::runtime::Result<ParagraphStyle> {
        let this = self;
        unsafe {
            let mut result__: ParagraphStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ParagraphStyle>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetStyle(&self, value: ParagraphStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SpaceAfter(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSpaceAfter(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SpaceBefore(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetSpaceBefore(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn WidowControl(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<FormatEffect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetWidowControl(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn TabCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn AddTab(
        &self,
        position: f32,
        align: TabAlignment,
        leader: TabLeader,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                position,
                align,
                leader,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ClearAllTabs(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn DeleteTab(&self, position: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                position,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetTab(
        &self,
        index: i32,
        position: &mut f32,
        align: &mut TabAlignment,
        leader: &mut TabLeader,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                index,
                position,
                align,
                leader,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(
        &self,
        format: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                format.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetClone<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(
        &self,
        format: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                format.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                start,
                left,
                right,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetLineSpacing(
        &self,
        rule: LineSpacingRule,
        spacing: f32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                rule,
                spacing,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextParagraphFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{219b6cdf-0d0b-5701-b8a1-6c906b3ebbe1}");
}
impl ::std::convert::From<ITextParagraphFormat> for ::windows::runtime::IUnknown {
    fn from(value: ITextParagraphFormat) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ITextParagraphFormat> for ::windows::runtime::IUnknown {
    fn from(value: &ITextParagraphFormat) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextParagraphFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ITextParagraphFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ITextParagraphFormat> for ::windows::runtime::IInspectable {
    fn from(value: ITextParagraphFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextParagraphFormat> for ::windows::runtime::IInspectable {
    fn from(value: &ITextParagraphFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ITextParagraphFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextParagraphFormat
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextParagraphFormat_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ParagraphAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ParagraphAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut LineSpacingRule,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MarkerAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: MarkerAlignment,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MarkerStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: MarkerStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MarkerType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: MarkerType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ParagraphStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ParagraphStyle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: FormatEffect,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: f32,
        align: TabAlignment,
        leader: TabLeader,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        position: *mut f32,
        align: *mut TabAlignment,
        leader: *mut TabLeader,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        start: f32,
        left: f32,
        right: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rule: LineSpacingRule,
        spacing: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Text`*"]
pub struct ITextRange(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextRange {
    type Vtable = ITextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        114600911,
        3078,
        23826,
        [167, 67, 133, 83, 126, 253, 9, 234],
    );
}
impl ITextRange {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacterFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetParagraphFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                format,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
        scanlength: i32,
        options: FindOptions,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                scanlength,
                options,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetCharacterUtf32(
        &self,
        value: &mut u32,
        offset: i32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
                offset,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetPoint(
        &self,
        horizontalalign: HorizontalCharacterAlignment,
        verticalalign: VerticalCharacterAlignment,
        options: PointOptions,
        point: &mut ::windows::Foundation::Point,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                horizontalalign,
                verticalalign,
                options,
                point,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetRect(
        &self,
        options: PointOptions,
        rect: &mut ::windows::Foundation::Rect,
        hit: &mut i32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                options,
                rect,
                hit,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut ::windows::runtime::HSTRING,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextGetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InsertImage<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        width: i32,
        height: i32,
        ascent: i32,
        verticalalign: VerticalCharacterAlignment,
        alternatetext: Param4,
        value: Param5,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                width,
                height,
                ascent,
                verticalalign,
                alternatetext.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                format,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetIndex(
        &self,
        unit: TextRangeUnit,
        index: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                unit,
                index,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Point>>(
        &self,
        point: Param0,
        options: PointOptions,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                options,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                startposition,
                endposition,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{06d4abcf-0c06-5d12-a743-85537efd09ea}");
}
impl ::std::convert::From<ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: ITextRange) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ITextRange> for ::windows::runtime::IInspectable {
    fn from(value: ITextRange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextRange> for ::windows::runtime::IInspectable {
    fn from(value: &ITextRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut RangeGravity,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: RangeGravity,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: i32,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: LetterCase,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        scanlength: i32,
        options: FindOptions,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut u32,
        offset: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        horizontalalign: HorizontalCharacterAlignment,
        verticalalign: VerticalCharacterAlignment,
        options: PointOptions,
        point: *mut ::windows::Foundation::Point,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: PointOptions,
        rect: *mut ::windows::Foundation::Rect,
        hit: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextGetOptions,
        value: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextGetOptions,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        range: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: i32,
        height: i32,
        ascent: i32,
        verticalalign: VerticalCharacterAlignment,
        alternatetext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        range: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        range: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        format: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: PointOptions,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        index: i32,
        extend: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        point: ::windows::Foundation::Point,
        options: PointOptions,
        extend: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        startposition: i32,
        endposition: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextSetOptions,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: TextSetOptions,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Text`*"]
pub struct ITextSelection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextSelection {
    type Vtable = ITextSelection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2405330097,
        11012,
        22687,
        [189, 36, 84, 229, 205, 141, 211, 153],
    );
}
impl ITextSelection {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<SelectionOptions> {
        let this = self;
        unsafe {
            let mut result__: SelectionOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SelectionOptions>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetOptions(&self, value: SelectionOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<SelectionType> {
        let this = self;
        unsafe {
            let mut result__: SelectionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SelectionType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveDown(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveLeft(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveRight(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveUp(
        &self,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn TypeText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacterFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetParagraphFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                format,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
        scanlength: i32,
        options: FindOptions,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                scanlength,
                options,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetCharacterUtf32(
        &self,
        value: &mut u32,
        offset: i32,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
                offset,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetPoint(
        &self,
        horizontalalign: HorizontalCharacterAlignment,
        verticalalign: VerticalCharacterAlignment,
        options: PointOptions,
        point: &mut ::windows::Foundation::Point,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                horizontalalign,
                verticalalign,
                options,
                point,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetRect(
        &self,
        options: PointOptions,
        rect: &mut ::windows::Foundation::Rect,
        hit: &mut i32,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                options,
                rect,
                hit,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut ::windows::runtime::HSTRING,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextGetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InsertImage<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        width: i32,
        height: i32,
        ascent: i32,
        verticalalign: VerticalCharacterAlignment,
        alternatetext: Param4,
        value: Param5,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                width,
                height,
                ascent,
                verticalalign,
                alternatetext.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                format,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetIndex(
        &self,
        unit: TextRangeUnit,
        index: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                unit,
                index,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Point>>(
        &self,
        point: Param0,
        options: PointOptions,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                options,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                startposition,
                endposition,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextSelection {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{8f5e6cb1-2b04-589f-bd24-54e5cd8dd399}");
}
impl ::std::convert::From<ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: ITextSelection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: &ITextSelection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ITextSelection> for ::windows::runtime::IInspectable {
    fn from(value: ITextSelection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextSelection> for ::windows::runtime::IInspectable {
    fn from(value: &ITextSelection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ITextSelection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITextSelection> for ITextRange {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITextSelection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITextSelection> for ITextRange {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITextSelection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::std::convert::TryInto::<ITextRange>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SelectionOptions,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SelectionOptions,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SelectionType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unit: TextRangeUnit,
        count: i32,
        extend: bool,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: LetterCase = LetterCase(0i32);
    pub const Upper: LetterCase = LetterCase(1i32);
}
impl ::std::convert::From<i32> for LetterCase {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LetterCase {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LetterCase {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LetterCase;i4)");
}
impl ::windows::runtime::DefaultType for LetterCase {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: LineSpacingRule = LineSpacingRule(0i32);
    pub const Single: LineSpacingRule = LineSpacingRule(1i32);
    pub const OneAndHalf: LineSpacingRule = LineSpacingRule(2i32);
    pub const Double: LineSpacingRule = LineSpacingRule(3i32);
    pub const AtLeast: LineSpacingRule = LineSpacingRule(4i32);
    pub const Exactly: LineSpacingRule = LineSpacingRule(5i32);
    pub const Multiple: LineSpacingRule = LineSpacingRule(6i32);
    pub const Percent: LineSpacingRule = LineSpacingRule(7i32);
}
impl ::std::convert::From<i32> for LineSpacingRule {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineSpacingRule {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineSpacingRule {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LineSpacingRule;i4)");
}
impl ::windows::runtime::DefaultType for LineSpacingRule {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: LinkType = LinkType(0i32);
    pub const NotALink: LinkType = LinkType(1i32);
    pub const ClientLink: LinkType = LinkType(2i32);
    pub const FriendlyLinkName: LinkType = LinkType(3i32);
    pub const FriendlyLinkAddress: LinkType = LinkType(4i32);
    pub const AutoLink: LinkType = LinkType(5i32);
    pub const AutoLinkEmail: LinkType = LinkType(6i32);
    pub const AutoLinkPhone: LinkType = LinkType(7i32);
    pub const AutoLinkPath: LinkType = LinkType(8i32);
}
impl ::std::convert::From<i32> for LinkType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LinkType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LinkType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.LinkType;i4)");
}
impl ::windows::runtime::DefaultType for LinkType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: MarkerAlignment = MarkerAlignment(0i32);
    pub const Left: MarkerAlignment = MarkerAlignment(1i32);
    pub const Center: MarkerAlignment = MarkerAlignment(2i32);
    pub const Right: MarkerAlignment = MarkerAlignment(3i32);
}
impl ::std::convert::From<i32> for MarkerAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerAlignment {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerAlignment;i4)");
}
impl ::windows::runtime::DefaultType for MarkerAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: MarkerStyle = MarkerStyle(0i32);
    pub const Parenthesis: MarkerStyle = MarkerStyle(1i32);
    pub const Parentheses: MarkerStyle = MarkerStyle(2i32);
    pub const Period: MarkerStyle = MarkerStyle(3i32);
    pub const Plain: MarkerStyle = MarkerStyle(4i32);
    pub const Minus: MarkerStyle = MarkerStyle(5i32);
    pub const NoNumber: MarkerStyle = MarkerStyle(6i32);
}
impl ::std::convert::From<i32> for MarkerStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerStyle;i4)");
}
impl ::windows::runtime::DefaultType for MarkerStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: MarkerType = MarkerType(0i32);
    pub const None: MarkerType = MarkerType(1i32);
    pub const Bullet: MarkerType = MarkerType(2i32);
    pub const Arabic: MarkerType = MarkerType(3i32);
    pub const LowercaseEnglishLetter: MarkerType = MarkerType(4i32);
    pub const UppercaseEnglishLetter: MarkerType = MarkerType(5i32);
    pub const LowercaseRoman: MarkerType = MarkerType(6i32);
    pub const UppercaseRoman: MarkerType = MarkerType(7i32);
    pub const UnicodeSequence: MarkerType = MarkerType(8i32);
    pub const CircledNumber: MarkerType = MarkerType(9i32);
    pub const BlackCircleWingding: MarkerType = MarkerType(10i32);
    pub const WhiteCircleWingding: MarkerType = MarkerType(11i32);
    pub const ArabicWide: MarkerType = MarkerType(12i32);
    pub const SimplifiedChinese: MarkerType = MarkerType(13i32);
    pub const TraditionalChinese: MarkerType = MarkerType(14i32);
    pub const JapanSimplifiedChinese: MarkerType = MarkerType(15i32);
    pub const JapanKorea: MarkerType = MarkerType(16i32);
    pub const ArabicDictionary: MarkerType = MarkerType(17i32);
    pub const ArabicAbjad: MarkerType = MarkerType(18i32);
    pub const Hebrew: MarkerType = MarkerType(19i32);
    pub const ThaiAlphabetic: MarkerType = MarkerType(20i32);
    pub const ThaiNumeric: MarkerType = MarkerType(21i32);
    pub const DevanagariVowel: MarkerType = MarkerType(22i32);
    pub const DevanagariConsonant: MarkerType = MarkerType(23i32);
    pub const DevanagariNumeric: MarkerType = MarkerType(24i32);
}
impl ::std::convert::From<i32> for MarkerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.MarkerType;i4)");
}
impl ::windows::runtime::DefaultType for MarkerType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: ParagraphAlignment = ParagraphAlignment(0i32);
    pub const Left: ParagraphAlignment = ParagraphAlignment(1i32);
    pub const Center: ParagraphAlignment = ParagraphAlignment(2i32);
    pub const Right: ParagraphAlignment = ParagraphAlignment(3i32);
    pub const Justify: ParagraphAlignment = ParagraphAlignment(4i32);
}
impl ::std::convert::From<i32> for ParagraphAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ParagraphAlignment {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ParagraphAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.ParagraphAlignment;i4)",
    );
}
impl ::windows::runtime::DefaultType for ParagraphAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: ParagraphStyle = ParagraphStyle(0i32);
    pub const None: ParagraphStyle = ParagraphStyle(1i32);
    pub const Normal: ParagraphStyle = ParagraphStyle(2i32);
    pub const Heading1: ParagraphStyle = ParagraphStyle(3i32);
    pub const Heading2: ParagraphStyle = ParagraphStyle(4i32);
    pub const Heading3: ParagraphStyle = ParagraphStyle(5i32);
    pub const Heading4: ParagraphStyle = ParagraphStyle(6i32);
    pub const Heading5: ParagraphStyle = ParagraphStyle(7i32);
    pub const Heading6: ParagraphStyle = ParagraphStyle(8i32);
    pub const Heading7: ParagraphStyle = ParagraphStyle(9i32);
    pub const Heading8: ParagraphStyle = ParagraphStyle(10i32);
    pub const Heading9: ParagraphStyle = ParagraphStyle(11i32);
}
impl ::std::convert::From<i32> for ParagraphStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ParagraphStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ParagraphStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.ParagraphStyle;i4)");
}
impl ::windows::runtime::DefaultType for ParagraphStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: PointOptions = PointOptions(0u32);
    pub const IncludeInset: PointOptions = PointOptions(1u32);
    pub const Start: PointOptions = PointOptions(32u32);
    pub const ClientCoordinates: PointOptions = PointOptions(256u32);
    pub const AllowOffClient: PointOptions = PointOptions(512u32);
    pub const Transform: PointOptions = PointOptions(1024u32);
    pub const NoHorizontalScroll: PointOptions = PointOptions(65536u32);
    pub const NoVerticalScroll: PointOptions = PointOptions(262144u32);
}
impl ::std::convert::From<u32> for PointOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PointOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.PointOptions;u4)");
}
impl ::windows::runtime::DefaultType for PointOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: RangeGravity = RangeGravity(0i32);
    pub const Backward: RangeGravity = RangeGravity(1i32);
    pub const Forward: RangeGravity = RangeGravity(2i32);
    pub const Inward: RangeGravity = RangeGravity(3i32);
    pub const Outward: RangeGravity = RangeGravity(4i32);
}
impl ::std::convert::From<i32> for RangeGravity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RangeGravity {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RangeGravity {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.RangeGravity;i4)");
}
impl ::windows::runtime::DefaultType for RangeGravity {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: RichEditMathMode = RichEditMathMode(0i32);
    pub const MathOnly: RichEditMathMode = RichEditMathMode(1i32);
}
impl ::std::convert::From<i32> for RichEditMathMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RichEditMathMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RichEditMathMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.RichEditMathMode;i4)");
}
impl ::windows::runtime::DefaultType for RichEditMathMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RichEditTextDocument(pub ::windows::runtime::IInspectable);
impl RichEditTextDocument {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CaretType(&self) -> ::windows::runtime::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__: CaretType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CaretType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCaretType(&self, value: CaretType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn DefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Selection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextSelection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndoLimit(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetUndoLimit(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanCopy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanPaste(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanRedo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanUndo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ApplyDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn BatchDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn BeginUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetDefaultCharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetDefaultParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetRange(
        &self,
        startposition: i32,
        endposition: i32,
    ) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                startposition,
                endposition,
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetRangeFromPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        point: Param0,
        options: PointOptions,
    ) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                options,
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut ::windows::runtime::HSTRING,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn LoadFromStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Redo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SaveToStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextGetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetDefaultCharacterFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetDefaultParagraphFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Undo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn AlignmentIncludesTrailingWhitespace(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetAlignmentIncludesTrailingWhitespace(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IgnoreTrailingCharacterSpacing(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ClearUndoRedoHistory(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RichEditTextDocument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Text.RichEditTextDocument;{1149d57d-86a6-59dd-88d9-196f27bc5c85})",
    );
}
unsafe impl ::windows::runtime::Interface for RichEditTextDocument {
    type Vtable = ITextDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        290051453,
        34470,
        23005,
        [136, 217, 25, 111, 39, 188, 92, 133],
    );
}
impl ::windows::runtime::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Microsoft.UI.Text.RichEditTextDocument";
}
impl ::std::convert::From<RichEditTextDocument> for ::windows::runtime::IUnknown {
    fn from(value: RichEditTextDocument) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&RichEditTextDocument> for ::windows::runtime::IUnknown {
    fn from(value: &RichEditTextDocument) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a RichEditTextDocument
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<RichEditTextDocument> for ::windows::runtime::IInspectable {
    fn from(value: RichEditTextDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RichEditTextDocument> for ::windows::runtime::IInspectable {
    fn from(value: &RichEditTextDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RichEditTextDocument
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RichEditTextDocument
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RichEditTextDocument {}
unsafe impl ::std::marker::Sync for RichEditTextDocument {}
#[doc = "*Required features: `UI_Text`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RichEditTextRange(pub ::windows::runtime::IInspectable);
impl RichEditTextRange {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextCharacterFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetCharacterFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RangeGravity>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextParagraphFormat>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetParagraphFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                format,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
        scanlength: i32,
        options: FindOptions,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                scanlength,
                options,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetCharacterUtf32(
        &self,
        value: &mut u32,
        offset: i32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value,
                offset,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ITextRange>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                unit,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetPoint(
        &self,
        horizontalalign: HorizontalCharacterAlignment,
        verticalalign: VerticalCharacterAlignment,
        options: PointOptions,
        point: &mut ::windows::Foundation::Point,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                horizontalalign,
                verticalalign,
                options,
                point,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetRect(
        &self,
        options: PointOptions,
        rect: &mut ::windows::Foundation::Rect,
        hit: &mut i32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                options,
                rect,
                hit,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetText(
        &self,
        options: TextGetOptions,
        value: &mut ::windows::runtime::HSTRING,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                options,
                value as *mut _ as _,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn GetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextGetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InsertImage<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        width: i32,
        height: i32,
        ascent: i32,
        verticalalign: VerticalCharacterAlignment,
        alternatetext: Param4,
        value: Param5,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                width,
                height,
                ascent,
                verticalalign,
                alternatetext.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(
        &self,
        range: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                range.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                format,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetIndex(
        &self,
        unit: TextRangeUnit,
        index: i32,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                unit,
                index,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Point>>(
        &self,
        point: Param0,
        options: PointOptions,
        extend: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                point.into_param().abi(),
                options,
                extend,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                startposition,
                endposition,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn SetTextViaStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        options: TextSetOptions,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                options,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                unit,
                extend,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RichEditTextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Text.RichEditTextRange;{06d4abcf-0c06-5d12-a743-85537efd09ea})",
    );
}
unsafe impl ::windows::runtime::Interface for RichEditTextRange {
    type Vtable = ITextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        114600911,
        3078,
        23826,
        [167, 67, 133, 83, 126, 253, 9, 234],
    );
}
impl ::windows::runtime::RuntimeName for RichEditTextRange {
    const NAME: &'static str = "Microsoft.UI.Text.RichEditTextRange";
}
impl ::std::convert::From<RichEditTextRange> for ::windows::runtime::IUnknown {
    fn from(value: RichEditTextRange) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&RichEditTextRange> for ::windows::runtime::IUnknown {
    fn from(value: &RichEditTextRange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<RichEditTextRange> for ::windows::runtime::IInspectable {
    fn from(value: RichEditTextRange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RichEditTextRange> for ::windows::runtime::IInspectable {
    fn from(value: &RichEditTextRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RichEditTextRange
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RichEditTextRange> for ITextRange {
    fn from(value: RichEditTextRange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RichEditTextRange> for ITextRange {
    fn from(value: &RichEditTextRange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
unsafe impl ::std::marker::Send for RichEditTextRange {}
unsafe impl ::std::marker::Sync for RichEditTextRange {}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: SelectionOptions = SelectionOptions(1u32);
    pub const AtEndOfLine: SelectionOptions = SelectionOptions(2u32);
    pub const Overtype: SelectionOptions = SelectionOptions(4u32);
    pub const Active: SelectionOptions = SelectionOptions(8u32);
    pub const Replace: SelectionOptions = SelectionOptions(16u32);
}
impl ::std::convert::From<u32> for SelectionOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelectionOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SelectionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.SelectionOptions;u4)");
}
impl ::windows::runtime::DefaultType for SelectionOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: SelectionType = SelectionType(0i32);
    pub const InsertionPoint: SelectionType = SelectionType(1i32);
    pub const Normal: SelectionType = SelectionType(2i32);
    pub const InlineShape: SelectionType = SelectionType(7i32);
    pub const Shape: SelectionType = SelectionType(8i32);
}
impl ::std::convert::From<i32> for SelectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelectionType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SelectionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.SelectionType;i4)");
}
impl ::windows::runtime::DefaultType for SelectionType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: TabAlignment = TabAlignment(0i32);
    pub const Center: TabAlignment = TabAlignment(1i32);
    pub const Right: TabAlignment = TabAlignment(2i32);
    pub const Decimal: TabAlignment = TabAlignment(3i32);
    pub const Bar: TabAlignment = TabAlignment(4i32);
}
impl ::std::convert::From<i32> for TabAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabAlignment {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TabAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TabAlignment;i4)");
}
impl ::windows::runtime::DefaultType for TabAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: TabLeader = TabLeader(0i32);
    pub const Dots: TabLeader = TabLeader(1i32);
    pub const Dashes: TabLeader = TabLeader(2i32);
    pub const Lines: TabLeader = TabLeader(3i32);
    pub const ThickLines: TabLeader = TabLeader(4i32);
    pub const Equals: TabLeader = TabLeader(5i32);
}
impl ::std::convert::From<i32> for TabLeader {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabLeader {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TabLeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TabLeader;i4)");
}
impl ::windows::runtime::DefaultType for TabLeader {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
pub struct TextConstants {}
impl TextConstants {
    #[doc = "*Required features: `UI_Text`*"]
    pub fn AutoColor() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MinUnitCount() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn MaxUnitCount() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndefinedColor() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndefinedFloatValue() -> ::windows::runtime::Result<f32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndefinedInt32Value() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndefinedFontStretch() -> ::windows::runtime::Result<::windows::UI::Text::FontStretch> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontStretch = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        })
    }
    #[doc = "*Required features: `UI_Text`*"]
    pub fn UndefinedFontStyle() -> ::windows::runtime::Result<::windows::UI::Text::FontStyle> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Text::FontStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        })
    }
    pub fn ITextConstantsStatics<
        R,
        F: FnOnce(&ITextConstantsStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextConstants, ITextConstantsStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for TextConstants {
    const NAME: &'static str = "Microsoft.UI.Text.TextConstants";
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: TextGetOptions = TextGetOptions(0u32);
    pub const AdjustCrlf: TextGetOptions = TextGetOptions(1u32);
    pub const UseCrlf: TextGetOptions = TextGetOptions(2u32);
    pub const UseObjectText: TextGetOptions = TextGetOptions(4u32);
    pub const AllowFinalEop: TextGetOptions = TextGetOptions(8u32);
    pub const NoHidden: TextGetOptions = TextGetOptions(32u32);
    pub const IncludeNumbering: TextGetOptions = TextGetOptions(64u32);
    pub const FormatRtf: TextGetOptions = TextGetOptions(8192u32);
    pub const UseLf: TextGetOptions = TextGetOptions(16777216u32);
}
impl ::std::convert::From<u32> for TextGetOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextGetOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextGetOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextGetOptions;u4)");
}
impl ::windows::runtime::DefaultType for TextGetOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: TextRangeUnit = TextRangeUnit(0i32);
    pub const Word: TextRangeUnit = TextRangeUnit(1i32);
    pub const Sentence: TextRangeUnit = TextRangeUnit(2i32);
    pub const Paragraph: TextRangeUnit = TextRangeUnit(3i32);
    pub const Line: TextRangeUnit = TextRangeUnit(4i32);
    pub const Story: TextRangeUnit = TextRangeUnit(5i32);
    pub const Screen: TextRangeUnit = TextRangeUnit(6i32);
    pub const Section: TextRangeUnit = TextRangeUnit(7i32);
    pub const Window: TextRangeUnit = TextRangeUnit(8i32);
    pub const CharacterFormat: TextRangeUnit = TextRangeUnit(9i32);
    pub const ParagraphFormat: TextRangeUnit = TextRangeUnit(10i32);
    pub const Object: TextRangeUnit = TextRangeUnit(11i32);
    pub const HardParagraph: TextRangeUnit = TextRangeUnit(12i32);
    pub const Cluster: TextRangeUnit = TextRangeUnit(13i32);
    pub const Bold: TextRangeUnit = TextRangeUnit(14i32);
    pub const Italic: TextRangeUnit = TextRangeUnit(15i32);
    pub const Underline: TextRangeUnit = TextRangeUnit(16i32);
    pub const Strikethrough: TextRangeUnit = TextRangeUnit(17i32);
    pub const ProtectedText: TextRangeUnit = TextRangeUnit(18i32);
    pub const Link: TextRangeUnit = TextRangeUnit(19i32);
    pub const SmallCaps: TextRangeUnit = TextRangeUnit(20i32);
    pub const AllCaps: TextRangeUnit = TextRangeUnit(21i32);
    pub const Hidden: TextRangeUnit = TextRangeUnit(22i32);
    pub const Outline: TextRangeUnit = TextRangeUnit(23i32);
    pub const Shadow: TextRangeUnit = TextRangeUnit(24i32);
    pub const Imprint: TextRangeUnit = TextRangeUnit(25i32);
    pub const Disabled: TextRangeUnit = TextRangeUnit(26i32);
    pub const Revised: TextRangeUnit = TextRangeUnit(27i32);
    pub const Subscript: TextRangeUnit = TextRangeUnit(28i32);
    pub const Superscript: TextRangeUnit = TextRangeUnit(29i32);
    pub const FontBound: TextRangeUnit = TextRangeUnit(30i32);
    pub const LinkProtected: TextRangeUnit = TextRangeUnit(31i32);
    pub const ContentLink: TextRangeUnit = TextRangeUnit(32i32);
}
impl ::std::convert::From<i32> for TextRangeUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextRangeUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextRangeUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextRangeUnit;i4)");
}
impl ::windows::runtime::DefaultType for TextRangeUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: TextScript = TextScript(0i32);
    pub const Ansi: TextScript = TextScript(1i32);
    pub const EastEurope: TextScript = TextScript(2i32);
    pub const Cyrillic: TextScript = TextScript(3i32);
    pub const Greek: TextScript = TextScript(4i32);
    pub const Turkish: TextScript = TextScript(5i32);
    pub const Hebrew: TextScript = TextScript(6i32);
    pub const Arabic: TextScript = TextScript(7i32);
    pub const Baltic: TextScript = TextScript(8i32);
    pub const Vietnamese: TextScript = TextScript(9i32);
    pub const Default: TextScript = TextScript(10i32);
    pub const Symbol: TextScript = TextScript(11i32);
    pub const Thai: TextScript = TextScript(12i32);
    pub const ShiftJis: TextScript = TextScript(13i32);
    pub const GB2312: TextScript = TextScript(14i32);
    pub const Hangul: TextScript = TextScript(15i32);
    pub const Big5: TextScript = TextScript(16i32);
    pub const PC437: TextScript = TextScript(17i32);
    pub const Oem: TextScript = TextScript(18i32);
    pub const Mac: TextScript = TextScript(19i32);
    pub const Armenian: TextScript = TextScript(20i32);
    pub const Syriac: TextScript = TextScript(21i32);
    pub const Thaana: TextScript = TextScript(22i32);
    pub const Devanagari: TextScript = TextScript(23i32);
    pub const Bengali: TextScript = TextScript(24i32);
    pub const Gurmukhi: TextScript = TextScript(25i32);
    pub const Gujarati: TextScript = TextScript(26i32);
    pub const Oriya: TextScript = TextScript(27i32);
    pub const Tamil: TextScript = TextScript(28i32);
    pub const Telugu: TextScript = TextScript(29i32);
    pub const Kannada: TextScript = TextScript(30i32);
    pub const Malayalam: TextScript = TextScript(31i32);
    pub const Sinhala: TextScript = TextScript(32i32);
    pub const Lao: TextScript = TextScript(33i32);
    pub const Tibetan: TextScript = TextScript(34i32);
    pub const Myanmar: TextScript = TextScript(35i32);
    pub const Georgian: TextScript = TextScript(36i32);
    pub const Jamo: TextScript = TextScript(37i32);
    pub const Ethiopic: TextScript = TextScript(38i32);
    pub const Cherokee: TextScript = TextScript(39i32);
    pub const Aboriginal: TextScript = TextScript(40i32);
    pub const Ogham: TextScript = TextScript(41i32);
    pub const Runic: TextScript = TextScript(42i32);
    pub const Khmer: TextScript = TextScript(43i32);
    pub const Mongolian: TextScript = TextScript(44i32);
    pub const Braille: TextScript = TextScript(45i32);
    pub const Yi: TextScript = TextScript(46i32);
    pub const Limbu: TextScript = TextScript(47i32);
    pub const TaiLe: TextScript = TextScript(48i32);
    pub const NewTaiLue: TextScript = TextScript(49i32);
    pub const SylotiNagri: TextScript = TextScript(50i32);
    pub const Kharoshthi: TextScript = TextScript(51i32);
    pub const Kayahli: TextScript = TextScript(52i32);
    pub const UnicodeSymbol: TextScript = TextScript(53i32);
    pub const Emoji: TextScript = TextScript(54i32);
    pub const Glagolitic: TextScript = TextScript(55i32);
    pub const Lisu: TextScript = TextScript(56i32);
    pub const Vai: TextScript = TextScript(57i32);
    pub const NKo: TextScript = TextScript(58i32);
    pub const Osmanya: TextScript = TextScript(59i32);
    pub const PhagsPa: TextScript = TextScript(60i32);
    pub const Gothic: TextScript = TextScript(61i32);
    pub const Deseret: TextScript = TextScript(62i32);
    pub const Tifinagh: TextScript = TextScript(63i32);
}
impl ::std::convert::From<i32> for TextScript {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextScript {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextScript {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextScript;i4)");
}
impl ::windows::runtime::DefaultType for TextScript {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: TextSetOptions = TextSetOptions(0u32);
    pub const UnicodeBidi: TextSetOptions = TextSetOptions(1u32);
    pub const Unlink: TextSetOptions = TextSetOptions(8u32);
    pub const Unhide: TextSetOptions = TextSetOptions(16u32);
    pub const CheckTextLimit: TextSetOptions = TextSetOptions(32u32);
    pub const FormatRtf: TextSetOptions = TextSetOptions(8192u32);
    pub const ApplyRtfDocumentDefaults: TextSetOptions = TextSetOptions(16384u32);
}
impl ::std::convert::From<u32> for TextSetOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextSetOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextSetOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.TextSetOptions;u4)");
}
impl ::windows::runtime::DefaultType for TextSetOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: UnderlineType = UnderlineType(0i32);
    pub const None: UnderlineType = UnderlineType(1i32);
    pub const Single: UnderlineType = UnderlineType(2i32);
    pub const Words: UnderlineType = UnderlineType(3i32);
    pub const Double: UnderlineType = UnderlineType(4i32);
    pub const Dotted: UnderlineType = UnderlineType(5i32);
    pub const Dash: UnderlineType = UnderlineType(6i32);
    pub const DashDot: UnderlineType = UnderlineType(7i32);
    pub const DashDotDot: UnderlineType = UnderlineType(8i32);
    pub const Wave: UnderlineType = UnderlineType(9i32);
    pub const Thick: UnderlineType = UnderlineType(10i32);
    pub const Thin: UnderlineType = UnderlineType(11i32);
    pub const DoubleWave: UnderlineType = UnderlineType(12i32);
    pub const HeavyWave: UnderlineType = UnderlineType(13i32);
    pub const LongDash: UnderlineType = UnderlineType(14i32);
    pub const ThickDash: UnderlineType = UnderlineType(15i32);
    pub const ThickDashDot: UnderlineType = UnderlineType(16i32);
    pub const ThickDashDotDot: UnderlineType = UnderlineType(17i32);
    pub const ThickDotted: UnderlineType = UnderlineType(18i32);
    pub const ThickLongDash: UnderlineType = UnderlineType(19i32);
}
impl ::std::convert::From<i32> for UnderlineType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnderlineType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnderlineType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Microsoft.UI.Text.UnderlineType;i4)");
}
impl ::windows::runtime::DefaultType for UnderlineType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Text`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: VerticalCharacterAlignment = VerticalCharacterAlignment(0i32);
    pub const Baseline: VerticalCharacterAlignment = VerticalCharacterAlignment(1i32);
    pub const Bottom: VerticalCharacterAlignment = VerticalCharacterAlignment(2i32);
}
impl ::std::convert::From<i32> for VerticalCharacterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VerticalCharacterAlignment {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VerticalCharacterAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Text.VerticalCharacterAlignment;i4)",
    );
}
impl ::windows::runtime::DefaultType for VerticalCharacterAlignment {
    type DefaultType = Self;
}
