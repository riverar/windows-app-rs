#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneBoundingBox(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneBoundingBox {
    type Vtable = ISceneBoundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        972769504,
        8554,
        22024,
        [145, 134, 107, 169, 249, 139, 92, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneBoundingBox_abi(
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponent {
    type Vtable = ISceneComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4147339723,
        32807,
        20706,
        [152, 238, 178, 227, 234, 5, 10, 84],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponent_abi(
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
        result__: *mut SceneComponentType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneComponentCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponentCollection {
    type Vtable = ISceneComponentCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3836877937,
        34786,
        23275,
        [133, 190, 136, 78, 131, 2, 39, 62],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentCollection_abi(
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
pub struct ISceneComponentFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneComponentFactory {
    type Vtable = ISceneComponentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        624986288,
        47807,
        20541,
        [154, 102, 13, 134, 175, 95, 115, 3],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentFactory_abi(
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
pub struct ISceneMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterial {
    type Vtable = ISceneMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        69288615,
        49003,
        22445,
        [186, 220, 245, 129, 243, 142, 219, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterial_abi(
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
pub struct ISceneMaterialFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialFactory {
    type Vtable = ISceneMaterialFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        628390035,
        34632,
        24416,
        [150, 159, 49, 143, 160, 183, 53, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialFactory_abi(
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
pub struct ISceneMaterialInput(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialInput {
    type Vtable = ISceneMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1147919070,
        29083,
        23988,
        [182, 153, 242, 38, 208, 6, 42, 46],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInput_abi(
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
pub struct ISceneMaterialInputFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMaterialInputFactory {
    type Vtable = ISceneMaterialInputFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3034234141,
        22720,
        22288,
        [146, 138, 188, 73, 176, 115, 86, 148],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory_abi(
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
pub struct ISceneMesh(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMesh {
    type Vtable = ISceneMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1559774890,
        62783,
        21854,
        [163, 173, 245, 188, 82, 202, 50, 251],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMesh_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        semantic: SceneAttributeSemantic,
        format: super::super::super::Graphics::DirectX::DirectXPixelFormat,
        memory: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        593544279,
        60846,
        22112,
        [190, 220, 137, 9, 101, 130, 237, 112],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap_abi(
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
pub struct ISceneMeshRendererComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3535701408,
        28840,
        23650,
        [132, 216, 139, 165, 94, 76, 100, 169],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshRendererComponentStatics {
    type Vtable = ISceneMeshRendererComponentStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3310324826,
        41220,
        23802,
        [137, 220, 19, 237, 170, 110, 61, 136],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMeshStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMeshStatics {
    type Vtable = ISceneMeshStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        700784933,
        38475,
        21269,
        [128, 249, 56, 147, 113, 50, 144, 245],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        172686580,
        31662,
        22274,
        [155, 133, 139, 200, 73, 243, 153, 135],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector4,
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneMetallicRoughnessMaterialStatics {
    type Vtable = ISceneMetallicRoughnessMaterialStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3757908035,
        43836,
        22432,
        [142, 19, 111, 9, 114, 94, 151, 15],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneModelTransform(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneModelTransform {
    type Vtable = ISceneModelTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1057314143,
        3943,
        22382,
        [157, 138, 147, 193, 242, 80, 194, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModelTransform_abi(
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
        result__: *mut ::windows::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Quaternion,
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneNode(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNode {
    type Vtable = ISceneNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2713510208,
        31170,
        23014,
        [155, 104, 99, 177, 186, 176, 226, 166],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNode_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SceneComponentType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneNodeCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNodeCollection {
    type Vtable = ISceneNodeCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4061771406,
        22118,
        23660,
        [170, 78, 8, 219, 7, 253, 107, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeCollection_abi(
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
pub struct ISceneNodeStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneNodeStatics {
    type Vtable = ISceneNodeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2149335956,
        16792,
        23969,
        [172, 57, 110, 138, 68, 181, 206, 87],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneObject(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneObject {
    type Vtable = ISceneObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1127474452,
        20423,
        21022,
        [139, 202, 17, 197, 31, 188, 175, 30],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObject_abi(
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
pub struct ISceneObjectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneObjectFactory {
    type Vtable = ISceneObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4000939901,
        30683,
        23628,
        [182, 245, 193, 147, 15, 173, 133, 197],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObjectFactory_abi(
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
pub struct IScenePbrMaterial(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScenePbrMaterial {
    type Vtable = IScenePbrMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        693962533,
        22270,
        22868,
        [128, 87, 63, 76, 167, 81, 91, 54],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterial_abi(
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
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SceneAlphaMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SceneAlphaMode,
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScenePbrMaterialFactory {
    type Vtable = IScenePbrMaterialFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2654262058,
        58124,
        20981,
        [132, 172, 100, 103, 149, 6, 5, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory_abi(
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
pub struct ISceneRendererComponent(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneRendererComponent {
    type Vtable = ISceneRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1806401584,
        35265,
        23996,
        [164, 142, 24, 5, 221, 249, 205, 209],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponent_abi(
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
pub struct ISceneRendererComponentFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneRendererComponentFactory {
    type Vtable = ISceneRendererComponentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1019920854,
        27151,
        22574,
        [187, 26, 16, 235, 193, 228, 5, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory_abi(
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
pub struct ISceneSurfaceMaterialInput(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3112520527,
        10348,
        20685,
        [167, 52, 73, 26, 37, 29, 95, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput_abi(
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
        result__: *mut super::CompositionBitmapInterpolationMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::CompositionBitmapInterpolationMode,
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
        result__: *mut SceneWrappingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SceneWrappingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SceneWrappingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SceneWrappingMode,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneSurfaceMaterialInputStatics {
    type Vtable = ISceneSurfaceMaterialInputStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2384177463,
        44384,
        20924,
        [130, 86, 202, 98, 196, 178, 174, 146],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneVisual(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneVisual {
    type Vtable = ISceneVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        21288877,
        27261,
        22987,
        [160, 249, 116, 160, 78, 133, 53, 44],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisual_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneVisualStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneVisualStatics {
    type Vtable = ISceneVisualStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2072880849,
        23512,
        20629,
        [146, 100, 229, 87, 38, 83, 234, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisualStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: SceneAlphaMode = SceneAlphaMode(0i32);
    pub const AlphaTest: SceneAlphaMode = SceneAlphaMode(1i32);
    pub const Blend: SceneAlphaMode = SceneAlphaMode(2i32);
}
impl ::std::convert::From<i32> for SceneAlphaMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneAlphaMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneAlphaMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneAlphaMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for SceneAlphaMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: SceneAttributeSemantic = SceneAttributeSemantic(0i32);
    pub const Vertex: SceneAttributeSemantic = SceneAttributeSemantic(1i32);
    pub const Normal: SceneAttributeSemantic = SceneAttributeSemantic(2i32);
    pub const TexCoord0: SceneAttributeSemantic = SceneAttributeSemantic(3i32);
    pub const TexCoord1: SceneAttributeSemantic = SceneAttributeSemantic(4i32);
    pub const Color: SceneAttributeSemantic = SceneAttributeSemantic(5i32);
    pub const Tangent: SceneAttributeSemantic = SceneAttributeSemantic(6i32);
}
impl ::std::convert::From<i32> for SceneAttributeSemantic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneAttributeSemantic {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneAttributeSemantic;i4)",
    );
}
impl ::windows::runtime::DefaultType for SceneAttributeSemantic {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneBoundingBox(pub ::windows::runtime::IInspectable);
impl SceneBoundingBox {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Center(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Extents(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Max(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Min(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneBoundingBox {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneBoundingBox;{39fb48e0-216a-5608-9186-6ba9f98b5c67})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        972769504,
        8554,
        22024,
        [145, 134, 107, 169, 249, 139, 92, 103],
    );
}
impl ::windows::runtime::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneBoundingBox";
}
impl ::std::convert::From<SceneBoundingBox> for ::windows::runtime::IUnknown {
    fn from(value: SceneBoundingBox) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneBoundingBox> for ::windows::runtime::IUnknown {
    fn from(value: &SceneBoundingBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneBoundingBox> for ::windows::runtime::IInspectable {
    fn from(value: SceneBoundingBox) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneBoundingBox> for ::windows::runtime::IInspectable {
    fn from(value: &SceneBoundingBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneBoundingBox
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneBoundingBox> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneBoundingBox> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneBoundingBox> for SceneObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneBoundingBox> for SceneObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneBoundingBox> for super::CompositionObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneBoundingBox> for super::CompositionObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneBoundingBox {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneBoundingBox {}
unsafe impl ::std::marker::Sync for SceneBoundingBox {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneComponent(pub ::windows::runtime::IInspectable);
impl SceneComponent {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__: SceneComponentType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneComponent {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneComponent;{f73361cb-8027-50e2-98ee-b2e3ea050a54})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneComponent {
    type Vtable = ISceneComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4147339723,
        32807,
        20706,
        [152, 238, 178, 227, 234, 5, 10, 84],
    );
}
impl ::windows::runtime::RuntimeName for SceneComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneComponent";
}
impl ::std::convert::From<SceneComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneComponent) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneComponent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneComponent> for SceneObject {
    fn from(value: SceneComponent) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneComponent> for SceneObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneComponent> for super::CompositionObject {
    fn from(value: SceneComponent) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneComponent> for super::CompositionObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneComponent {}
unsafe impl ::std::marker::Sync for SceneComponent {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneComponentCollection(pub ::windows::runtime::IInspectable);
impl SceneComponentCollection {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<SceneComponent>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVectorView<SceneComponent>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<SceneComponent>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, SceneComponent>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneComponent>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneComponent>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, SceneComponent>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<SceneComponent as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::std::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ReplaceAll(
        &self,
        items: &[<SceneComponent as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                items.len() as u32,
                ::std::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IIterator<SceneComponent>>
    {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<SceneComponent>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<SceneComponent>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneComponentCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneComponentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Composition.Scenes.SceneComponent;{f73361cb-8027-50e2-98ee-b2e3ea050a54})))" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneComponentCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<SceneComponent>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < SceneComponent > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::runtime::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneComponentCollection";
}
impl ::std::convert::From<SceneComponentCollection> for ::windows::runtime::IUnknown {
    fn from(value: SceneComponentCollection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneComponentCollection> for ::windows::runtime::IUnknown {
    fn from(value: &SceneComponentCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneComponentCollection> for ::windows::runtime::IInspectable {
    fn from(value: SceneComponentCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneComponentCollection> for ::windows::runtime::IInspectable {
    fn from(value: &SceneComponentCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SceneComponentCollection>
    for ::windows::Foundation::Collections::IVector<SceneComponent>
{
    fn from(value: SceneComponentCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SceneComponentCollection>
    for ::windows::Foundation::Collections::IVector<SceneComponent>
{
    fn from(value: &SceneComponentCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a>
    ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<SceneComponent>>
    for SceneComponentCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<SceneComponent>>
    {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a>
    ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<SceneComponent>>
    for &SceneComponentCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<SceneComponent>>
    {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<SceneComponentCollection>
    for ::windows::Foundation::Collections::IIterable<SceneComponent>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneComponentCollection>
    for ::windows::Foundation::Collections::IIterable<SceneComponent>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<SceneComponent>>
    for SceneComponentCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<SceneComponent>>
    {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<SceneComponent>>
    for &SceneComponentCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<SceneComponent>>
    {
        :: std :: convert :: TryInto :: < ::windows::Foundation::Collections:: IIterable :: < SceneComponent > > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
impl ::std::convert::TryFrom<SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneComponentCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneComponentCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneComponentCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneComponentCollection> for SceneObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneComponentCollection> for SceneObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneComponentCollection> for super::CompositionObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneComponentCollection> for super::CompositionObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneComponentCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneComponentCollection {}
unsafe impl ::std::marker::Sync for SceneComponentCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::std::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: SceneComponentType = SceneComponentType(0i32);
}
impl ::std::convert::From<i32> for SceneComponentType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneComponentType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneComponentType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneComponentType;i4)",
    );
}
impl ::windows::runtime::DefaultType for SceneComponentType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMaterial(pub ::windows::runtime::IInspectable);
impl SceneMaterial {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneMaterial;{042142a7-bf6b-57ad-badc-f581f38edb48})",
    );
}
unsafe impl ::windows::runtime::Interface for SceneMaterial {
    type Vtable = ISceneMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        69288615,
        49003,
        22445,
        [186, 220, 245, 129, 243, 142, 219, 72],
    );
}
impl ::windows::runtime::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMaterial";
}
impl ::std::convert::From<SceneMaterial> for ::windows::runtime::IUnknown {
    fn from(value: SceneMaterial) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMaterial> for ::windows::runtime::IInspectable {
    fn from(value: SceneMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMaterial> for SceneObject {
    fn from(value: SceneMaterial) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMaterial> for SceneObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMaterial> for super::CompositionObject {
    fn from(value: SceneMaterial) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMaterial> for super::CompositionObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMaterial {}
unsafe impl ::std::marker::Sync for SceneMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMaterialInput(pub ::windows::runtime::IInspectable);
impl SceneMaterialInput {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMaterialInput {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMaterialInput;{446bdade-719b-5db4-b699-f226d0062a2e})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1147919070,
        29083,
        23988,
        [182, 153, 242, 38, 208, 6, 42, 46],
    );
}
impl ::windows::runtime::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMaterialInput";
}
impl ::std::convert::From<SceneMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: SceneMaterialInput) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMaterialInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: SceneMaterialInput) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMaterialInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMaterialInput> for SceneObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMaterialInput> for SceneObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMaterialInput> for super::CompositionObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMaterialInput> for super::CompositionObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMaterialInput {}
unsafe impl ::std::marker::Sync for SceneMaterialInput {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMesh(pub ::windows::runtime::IInspectable);
impl SceneMesh {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Bounds(&self) -> ::windows::runtime::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneBoundingBox>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Graphics_DirectX`*"]
    pub fn PrimitiveTopology(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Graphics_DirectX`*"]
    pub fn SetPrimitiveTopology(
        &self,
        value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `Graphics_DirectX`*"]
    pub fn FillMeshAttribute<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::Foundation::MemoryBuffer>,
    >(
        &self,
        semantic: SceneAttributeSemantic,
        format: super::super::super::Graphics::DirectX::DirectXPixelFormat,
        memory: Param2,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                semantic,
                format,
                memory.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneMesh> {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneMesh>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneMesh, ISceneMeshStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMesh {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneMesh;{5cf846aa-f53f-555e-a3ad-f5bc52ca32fb})",
    );
}
unsafe impl ::windows::runtime::Interface for SceneMesh {
    type Vtable = ISceneMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1559774890,
        62783,
        21854,
        [163, 173, 245, 188, 82, 202, 50, 251],
    );
}
impl ::windows::runtime::RuntimeName for SceneMesh {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMesh";
}
impl ::std::convert::From<SceneMesh> for ::windows::runtime::IUnknown {
    fn from(value: SceneMesh) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMesh> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMesh) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMesh> for ::windows::runtime::IInspectable {
    fn from(value: SceneMesh) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMesh> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMesh) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMesh> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMesh) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMesh> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMesh) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMesh) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMesh) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMesh> for SceneObject {
    fn from(value: SceneMesh) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMesh> for SceneObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMesh> for super::CompositionObject {
    fn from(value: SceneMesh) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMesh> for super::CompositionObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMesh {}
unsafe impl ::std::marker::Sync for SceneMesh {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMeshMaterialAttributeMap(pub ::windows::runtime::IInspectable);
impl SceneMeshMaterialAttributeMap {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    SceneAttributeSemantic,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    SceneAttributeSemantic,
                >,
            >>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        key: Param0,
    ) -> ::windows::runtime::Result<SceneAttributeSemantic> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__: SceneAttributeSemantic = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                key.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneAttributeSemantic>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        key: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                key.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    > {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        key: Param0,
        value: SceneAttributeSemantic,
    ) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                key.into_param().abi(),
                value,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        key: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                key.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMeshMaterialAttributeMap;{2360c457-edae-5660-bedc-89096582ed70})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        593544279,
        60846,
        22112,
        [190, 220, 137, 9, 101, 130, 237, 112],
    );
}
impl ::windows::runtime::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
impl ::std::convert::From<SceneMeshMaterialAttributeMap> for ::windows::runtime::IUnknown {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMeshMaterialAttributeMap> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMeshMaterialAttributeMap> for ::windows::runtime::IInspectable {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMeshMaterialAttributeMap> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > for SceneMeshMaterialAttributeMap
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > for &SceneMeshMaterialAttributeMap
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > {
        ::std::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    SceneAttributeSemantic,
                >,
            >,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IMap<
        ::windows::runtime::HSTRING,
        SceneAttributeSemantic,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IMap<
        ::windows::runtime::HSTRING,
        SceneAttributeSemantic,
    >
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    > for SceneMeshMaterialAttributeMap
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    > for &SceneMeshMaterialAttributeMap
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            SceneAttributeSemantic,
        >,
    > {
        ::std::convert::TryInto::<
            ::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                SceneAttributeSemantic,
            >,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMeshMaterialAttributeMap> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshMaterialAttributeMap> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMeshMaterialAttributeMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &SceneMeshMaterialAttributeMap
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMeshMaterialAttributeMap {}
unsafe impl ::std::marker::Sync for SceneMeshMaterialAttributeMap {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        SceneAttributeSemantic,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::runtime::HSTRING,
        SceneAttributeSemantic,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMeshRendererComponent(pub ::windows::runtime::IInspectable);
impl SceneMeshRendererComponent {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Material(&self) -> ::windows::runtime::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterial>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMaterial<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterial>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Mesh(&self) -> ::windows::runtime::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMesh>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMesh<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMesh>>(
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn UVMappings(&self) -> ::windows::runtime::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMeshMaterialAttributeMap>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneMeshRendererComponent> {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneMeshRendererComponent>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = &::windows::runtime::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__: SceneComponentType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneMeshRendererComponentStatics<
        R,
        F: FnOnce(&ISceneMeshRendererComponentStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SceneMeshRendererComponent,
            ISceneMeshRendererComponentStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMeshRendererComponent;{d2be85a0-70a8-5c62-84d8-8ba55e4c64a9})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3535701408,
        28840,
        23650,
        [132, 216, 139, 165, 94, 76, 100, 169],
    );
}
impl ::windows::runtime::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMeshRendererComponent";
}
impl ::std::convert::From<SceneMeshRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneMeshRendererComponent) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMeshRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneMeshRendererComponent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMeshRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMeshRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::std::convert::Into::<SceneRendererComponent>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneRendererComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneRendererComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneRendererComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneRendererComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneRendererComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneRendererComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMeshRendererComponent> for SceneComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::std::convert::Into::<SceneComponent>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for SceneComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMeshRendererComponent> for SceneObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for SceneObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMeshRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &SceneMeshRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMeshRendererComponent {}
unsafe impl ::std::marker::Sync for SceneMeshRendererComponent {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneMetallicRoughnessMaterial(pub ::windows::runtime::IInspectable);
impl SceneMetallicRoughnessMaterial {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BaseColorInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBaseColorInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BaseColorFactor(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector4 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector4>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBaseColorFactor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector4>,
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn MetallicFactor(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMetallicFactor(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn MetallicRoughnessInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetMetallicRoughnessInput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RoughnessFactor(&self) -> ::windows::runtime::Result<f32> {
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRoughnessFactor(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneMetallicRoughnessMaterial> {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneMetallicRoughnessMaterial>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaCutoff(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaMode(&self) -> ::windows::runtime::Result<SceneAlphaMode> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: SceneAlphaMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveFactor(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveFactor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsDoubleSided(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalScale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionStrength(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneMetallicRoughnessMaterialStatics<
        R,
        F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SceneMetallicRoughnessMaterial,
            ISceneMetallicRoughnessMaterialStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMetallicRoughnessMaterial;{0a4afcf4-7bae-5702-9b85-8bc849f39987})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        172686580,
        31662,
        22274,
        [155, 133, 139, 200, 73, 243, 153, 135],
    );
}
impl ::windows::runtime::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for ::windows::runtime::IUnknown {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for ::windows::runtime::IInspectable {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneMetallicRoughnessMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneMetallicRoughnessMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::std::convert::Into::<ScenePbrMaterial>::into(&value)
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ScenePbrMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ScenePbrMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ScenePbrMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ScenePbrMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ScenePbrMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ScenePbrMaterial>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::std::convert::Into::<SceneMaterial>::into(&value)
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterial>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneMetallicRoughnessMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &SceneMetallicRoughnessMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneMetallicRoughnessMaterial {}
unsafe impl ::std::marker::Sync for SceneMetallicRoughnessMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneModelTransform(pub ::windows::runtime::IInspectable);
impl SceneModelTransform {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Orientation(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Quaternion>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngle(&self) -> ::windows::runtime::Result<f32> {
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngleInDegrees(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAxis(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetScale<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Translation(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetTranslation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneModelTransform {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneModelTransform;{3f05555f-0f67-576e-9d8a-93c1f250c29f})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneModelTransform {
    type Vtable = ISceneModelTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1057314143,
        3943,
        22382,
        [157, 138, 147, 193, 242, 80, 194, 159],
    );
}
impl ::windows::runtime::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneModelTransform";
}
impl ::std::convert::From<SceneModelTransform> for ::windows::runtime::IUnknown {
    fn from(value: SceneModelTransform) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneModelTransform> for ::windows::runtime::IUnknown {
    fn from(value: &SceneModelTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneModelTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneModelTransform> for ::windows::runtime::IInspectable {
    fn from(value: SceneModelTransform) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneModelTransform> for ::windows::runtime::IInspectable {
    fn from(value: &SceneModelTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneModelTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneModelTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneModelTransform> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneModelTransform> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneModelTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneModelTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneModelTransform> for super::CompositionTransform {
    fn from(value: SceneModelTransform) -> Self {
        ::std::convert::Into::<super::CompositionTransform>::into(&value)
    }
}
impl ::std::convert::From<&SceneModelTransform> for super::CompositionTransform {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionTransform> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionTransform>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionTransform> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneModelTransform> for super::CompositionObject {
    fn from(value: SceneModelTransform) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneModelTransform> for super::CompositionObject {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneModelTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneModelTransform {}
unsafe impl ::std::marker::Sync for SceneModelTransform {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneNode(pub ::windows::runtime::IInspectable);
impl SceneNode {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Children(&self) -> ::windows::runtime::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneNodeCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Components(&self) -> ::windows::runtime::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneComponentCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Transform(&self) -> ::windows::runtime::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneModelTransform>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn FindFirstComponentOfType(
        &self,
        value: SceneComponentType,
    ) -> ::windows::runtime::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<SceneComponent>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneNode> {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneNode>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneNode, ISceneNodeStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneNode;{a1bce140-79c2-59e6-9b68-63b1bab0e2a6})",
    );
}
unsafe impl ::windows::runtime::Interface for SceneNode {
    type Vtable = ISceneNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2713510208,
        31170,
        23014,
        [155, 104, 99, 177, 186, 176, 226, 166],
    );
}
impl ::windows::runtime::RuntimeName for SceneNode {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneNode";
}
impl ::std::convert::From<SceneNode> for ::windows::runtime::IUnknown {
    fn from(value: SceneNode) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneNode> for ::windows::runtime::IUnknown {
    fn from(value: &SceneNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneNode> for ::windows::runtime::IInspectable {
    fn from(value: SceneNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneNode> for ::windows::runtime::IInspectable {
    fn from(value: &SceneNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneNode> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneNode> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneNode> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneNode> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneNode> for SceneObject {
    fn from(value: SceneNode) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneNode> for SceneObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneNode> for super::CompositionObject {
    fn from(value: SceneNode) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneNode> for super::CompositionObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneNode {}
unsafe impl ::std::marker::Sync for SceneNode {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneNodeCollection(pub ::windows::runtime::IInspectable);
impl SceneNodeCollection {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVectorView<SceneNode>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<SceneNode>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneNode>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn InsertAt<'a, Param1: ::windows::runtime::IntoParam<'a, SceneNode>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Append<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<SceneNode as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                startindex,
                items.len() as u32,
                ::std::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ReplaceAll(
        &self,
        items: &[<SceneNode as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                items.len() as u32,
                ::std::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IIterator<SceneNode>> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<SceneNode>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<SceneNode>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneNodeCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneNodeCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Composition.Scenes.SceneNode;{a1bce140-79c2-59e6-9b68-63b1bab0e2a6})))" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneNodeCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_abi<SceneNode>;
    const IID : :: windows :: runtime :: GUID = :: windows :: runtime :: GUID :: from_signature ( < ::windows::Foundation::Collections:: IVector :: < SceneNode > as :: windows :: runtime :: RuntimeType > :: SIGNATURE ) ;
}
impl ::windows::runtime::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneNodeCollection";
}
impl ::std::convert::From<SceneNodeCollection> for ::windows::runtime::IUnknown {
    fn from(value: SceneNodeCollection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneNodeCollection> for ::windows::runtime::IUnknown {
    fn from(value: &SceneNodeCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneNodeCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneNodeCollection> for ::windows::runtime::IInspectable {
    fn from(value: SceneNodeCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneNodeCollection> for ::windows::runtime::IInspectable {
    fn from(value: &SceneNodeCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneNodeCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneNodeCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SceneNodeCollection>
    for ::windows::Foundation::Collections::IVector<SceneNode>
{
    fn from(value: SceneNodeCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SceneNodeCollection>
    for ::windows::Foundation::Collections::IVector<SceneNode>
{
    fn from(value: &SceneNodeCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<SceneNode>>
    for SceneNodeCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<SceneNode>> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IVector<SceneNode>>
    for &SceneNodeCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IVector<SceneNode>> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<SceneNodeCollection>
    for ::windows::Foundation::Collections::IIterable<SceneNode>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneNodeCollection>
    for ::windows::Foundation::Collections::IIterable<SceneNode>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<SceneNode>>
    for SceneNodeCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<SceneNode>>
    {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::Collections::IIterable<SceneNode>>
    for &SceneNodeCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Foundation::Collections::IIterable<SceneNode>>
    {
        :: std :: convert :: TryInto :: < ::windows::Foundation::Collections:: IIterable :: < SceneNode > > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
impl ::std::convert::TryFrom<SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneNodeCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneNodeCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneNodeCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneNodeCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneNodeCollection> for SceneObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneNodeCollection> for SceneObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneNodeCollection> for super::CompositionObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneNodeCollection> for super::CompositionObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneNodeCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneNodeCollection {}
unsafe impl ::std::marker::Sync for SceneNodeCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::std::convert::TryInto::try_into(self).ok(),
        )
    }
}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneObject(pub ::windows::runtime::IInspectable);
impl SceneObject {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneObject;{4333e514-4fc7-521e-8bca-11c51fbcaf1e})",
    );
}
unsafe impl ::windows::runtime::Interface for SceneObject {
    type Vtable = ISceneObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1127474452,
        20423,
        21022,
        [139, 202, 17, 197, 31, 188, 175, 30],
    );
}
impl ::windows::runtime::RuntimeName for SceneObject {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneObject";
}
impl ::std::convert::From<SceneObject> for ::windows::runtime::IUnknown {
    fn from(value: SceneObject) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneObject> for ::windows::runtime::IUnknown {
    fn from(value: &SceneObject) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneObject> for ::windows::runtime::IInspectable {
    fn from(value: SceneObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneObject> for ::windows::runtime::IInspectable {
    fn from(value: &SceneObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneObject> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneObject) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneObject> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneObject) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneObject) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneObject) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneObject> for super::CompositionObject {
    fn from(value: SceneObject) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneObject> for super::CompositionObject {
    fn from(value: &SceneObject) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneObject {}
unsafe impl ::std::marker::Sync for SceneObject {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScenePbrMaterial(pub ::windows::runtime::IInspectable);
impl ScenePbrMaterial {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaCutoff(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AlphaMode(&self) -> ::windows::runtime::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: SceneAlphaMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn EmissiveFactor(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetEmissiveFactor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsDoubleSided(&self) -> ::windows::runtime::Result<bool> {
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn NormalScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetNormalScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionInput(&self) -> ::windows::runtime::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionInput<'a, Param0: ::windows::runtime::IntoParam<'a, SceneMaterialInput>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn OcclusionStrength(&self) -> ::windows::runtime::Result<f32> {
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScenePbrMaterial {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.ScenePbrMaterial;{295d0725-56fe-5954-8057-3f4ca7515b36})" ) ;
}
unsafe impl ::windows::runtime::Interface for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        693962533,
        22270,
        22868,
        [128, 87, 63, 76, 167, 81, 91, 54],
    );
}
impl ::windows::runtime::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.ScenePbrMaterial";
}
impl ::std::convert::From<ScenePbrMaterial> for ::windows::runtime::IUnknown {
    fn from(value: ScenePbrMaterial) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ScenePbrMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &ScenePbrMaterial) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ScenePbrMaterial> for ::windows::runtime::IInspectable {
    fn from(value: ScenePbrMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScenePbrMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &ScenePbrMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ScenePbrMaterial
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<ScenePbrMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ScenePbrMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<ScenePbrMaterial> for SceneMaterial {
    fn from(value: ScenePbrMaterial) -> Self {
        ::std::convert::Into::<SceneMaterial>::into(&value)
    }
}
impl ::std::convert::From<&ScenePbrMaterial> for SceneMaterial {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterial>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterial> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterial> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterial>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ScenePbrMaterial> for SceneObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&ScenePbrMaterial> for SceneObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<ScenePbrMaterial> for super::CompositionObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&ScenePbrMaterial> for super::CompositionObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &ScenePbrMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for ScenePbrMaterial {}
unsafe impl ::std::marker::Sync for ScenePbrMaterial {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneRendererComponent(pub ::windows::runtime::IInspectable);
impl SceneRendererComponent {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ComponentType(&self) -> ::windows::runtime::Result<SceneComponentType> {
        let this = &::windows::runtime::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__: SceneComponentType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneRendererComponent {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneRendererComponent;{6bab8030-89c1-5dbc-a48e-1805ddf9cdd1})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1806401584,
        35265,
        23996,
        [164, 142, 24, 5, 221, 249, 205, 209],
    );
}
impl ::windows::runtime::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneRendererComponent";
}
impl ::std::convert::From<SceneRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: SceneRendererComponent) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneRendererComponent> for ::windows::runtime::IUnknown {
    fn from(value: &SceneRendererComponent) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: SceneRendererComponent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneRendererComponent> for ::windows::runtime::IInspectable {
    fn from(value: &SceneRendererComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneRendererComponent
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneRendererComponent> for SceneComponent {
    fn from(value: SceneRendererComponent) -> Self {
        ::std::convert::Into::<SceneComponent>::into(&value)
    }
}
impl ::std::convert::From<&SceneRendererComponent> for SceneComponent {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneComponent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneComponent> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneComponent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneComponent>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneRendererComponent> for SceneObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneRendererComponent> for SceneObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneRendererComponent> for super::CompositionObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneRendererComponent> for super::CompositionObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneRendererComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneRendererComponent {}
unsafe impl ::std::marker::Sync for SceneRendererComponent {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneSurfaceMaterialInput(pub ::windows::runtime::IInspectable);
impl SceneSurfaceMaterialInput {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BitmapInterpolationMode(
        &self,
    ) -> ::windows::runtime::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__: super::CompositionBitmapInterpolationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionBitmapInterpolationMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBitmapInterpolationMode(
        &self,
        value: super::CompositionBitmapInterpolationMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Surface(&self) -> ::windows::runtime::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ICompositionSurface>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::ICompositionSurface>>(
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
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn WrappingUMode(&self) -> ::windows::runtime::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__: SceneWrappingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneWrappingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn WrappingVMode(&self) -> ::windows::runtime::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__: SceneWrappingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneWrappingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneSurfaceMaterialInput> {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneSurfaceMaterialInput>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneSurfaceMaterialInputStatics<
        R,
        F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SceneSurfaceMaterialInput,
            ISceneSurfaceMaterialInputStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneSurfaceMaterialInput;{b9854b4f-286c-50cd-a734-491a251d5fd3})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3112520527,
        10348,
        20685,
        [167, 52, 73, 26, 37, 29, 95, 211],
    );
}
impl ::windows::runtime::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
impl ::std::convert::From<SceneSurfaceMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneSurfaceMaterialInput> for ::windows::runtime::IUnknown {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneSurfaceMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneSurfaceMaterialInput> for ::windows::runtime::IInspectable {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneSurfaceMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneSurfaceMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::std::convert::Into::<SceneMaterialInput>::into(&value)
    }
}
impl ::std::convert::From<&SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterialInput> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterialInput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterialInput>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneMaterialInput> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneMaterialInput> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneMaterialInput>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::std::convert::Into::<SceneObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, SceneObject> for &SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, SceneObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<SceneObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneSurfaceMaterialInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &SceneSurfaceMaterialInput
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneSurfaceMaterialInput {}
unsafe impl ::std::marker::Sync for SceneSurfaceMaterialInput {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SceneVisual(pub ::windows::runtime::IInspectable);
impl SceneVisual {
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Root(&self) -> ::windows::runtime::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRoot<'a, Param0: ::windows::runtime::IntoParam<'a, SceneNode>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<SceneVisual> {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SceneVisual>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Scenes`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Children(&self) -> ::windows::runtime::Result<super::VisualCollection> {
        let this = &::windows::runtime::Interface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::VisualCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn AnchorPoint(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetAnchorPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BackfaceVisibility(
        &self,
    ) -> ::windows::runtime::Result<super::CompositionBackfaceVisibility> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionBackfaceVisibility = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionBackfaceVisibility>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBackfaceVisibility(
        &self,
        value: super::CompositionBackfaceVisibility,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn BorderMode(&self) -> ::windows::runtime::Result<super::CompositionBorderMode> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionBorderMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionBorderMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetBorderMode(
        &self,
        value: super::CompositionBorderMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn CenterPoint(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetCenterPoint<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Clip(&self) -> ::windows::runtime::Result<super::CompositionClip> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionClip>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, super::CompositionClip>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn CompositeMode(&self) -> ::windows::runtime::Result<super::CompositionCompositeMode> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: super::CompositionCompositeMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionCompositeMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetCompositeMode(
        &self,
        value: super::CompositionCompositeMode,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Offset(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOffset<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOpacity(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Orientation(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Quaternion> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Quaternion>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Parent(&self) -> ::windows::runtime::Result<super::ContainerVisual> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ContainerVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngle(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAngleInDegrees(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RotationAxis(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRotationAxis<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetScale<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetSize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Matrix4x4 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetTransformMatrix<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Matrix4x4>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn ParentForTransform(&self) -> ::windows::runtime::Result<super::Visual> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetParentForTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RelativeOffsetAdjustment(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRelativeOffsetAdjustment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn RelativeSizeAdjustment(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetRelativeSizeAdjustment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsHitTestVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn IsPixelSnappingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Scenes`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ISceneVisualStatics<
        R,
        F: FnOnce(&ISceneVisualStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneVisual, ISceneVisualStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneVisual {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneVisual;{0144d7ad-6a7d-59cb-a0f9-74a04e85352c})",
    );
}
unsafe impl ::windows::runtime::Interface for SceneVisual {
    type Vtable = ISceneVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        21288877,
        27261,
        22987,
        [160, 249, 116, 160, 78, 133, 53, 44],
    );
}
impl ::windows::runtime::RuntimeName for SceneVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneVisual";
}
impl ::std::convert::From<SceneVisual> for ::windows::runtime::IUnknown {
    fn from(value: SceneVisual) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SceneVisual> for ::windows::runtime::IUnknown {
    fn from(value: &SceneVisual) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SceneVisual> for ::windows::runtime::IInspectable {
    fn from(value: SceneVisual) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SceneVisual> for ::windows::runtime::IInspectable {
    fn from(value: &SceneVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SceneVisual> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneVisual) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneVisual> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneVisual) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SceneVisual> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneVisual) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SceneVisual> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneVisual) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<SceneVisual> for super::ContainerVisual {
    fn from(value: SceneVisual) -> Self {
        ::std::convert::Into::<super::ContainerVisual>::into(&value)
    }
}
impl ::std::convert::From<&SceneVisual> for super::ContainerVisual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ContainerVisual> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ContainerVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ContainerVisual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ContainerVisual> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ContainerVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ContainerVisual>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneVisual> for super::Visual {
    fn from(value: SceneVisual) -> Self {
        ::std::convert::Into::<super::Visual>::into(&value)
    }
}
impl ::std::convert::From<&SceneVisual> for super::Visual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::Visual> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Visual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Visual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::Visual> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Visual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Visual>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<SceneVisual> for super::CompositionObject {
    fn from(value: SceneVisual) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&SceneVisual> for super::CompositionObject {
    fn from(value: &SceneVisual) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &SceneVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for SceneVisual {}
unsafe impl ::std::marker::Sync for SceneVisual {}
#[doc = "*Required features: `UI_Composition_Scenes`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: SceneWrappingMode = SceneWrappingMode(0i32);
    pub const MirroredRepeat: SceneWrappingMode = SceneWrappingMode(1i32);
    pub const Repeat: SceneWrappingMode = SceneWrappingMode(2i32);
}
impl ::std::convert::From<i32> for SceneWrappingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneWrappingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneWrappingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneWrappingMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for SceneWrappingMode {
    type DefaultType = Self;
}