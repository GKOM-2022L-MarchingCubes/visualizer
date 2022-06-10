use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
            CompareFunction, RenderPipelineDescriptor, SpecializedMeshPipelineError,
        },
        renderer::RenderDevice,
    },
};

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "b6459ba2-e8cb-11ec-8fea-0242ac120002"]
pub struct SkyboxMaterial {
    pub texture: Handle<Image>,
}

#[derive(Clone)]
pub struct GpuSkyboxMaterial {
    pub bind_group: BindGroup,
    pub texture: Handle<Image>,
}

impl RenderAsset for SkyboxMaterial {
    type ExtractedAsset = SkyboxMaterial;
    type PreparedAsset = GpuSkyboxMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuSkyboxMaterial {
            bind_group,
            texture: material.texture,
        })
    }
}

impl SpecializedMaterial for SkyboxMaterial {
    type Key = ();

    fn key(_: &<SkyboxMaterial as RenderAsset>::PreparedAsset) -> Self::Key {}

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/sky.very"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/sky.frag"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[],
            label: None,
        })
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _: Self::Key,
        _layout: &MeshVertexBufferLayout,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor
            .depth_stencil
            .as_mut()
            .map(|mut depth_stencil_state| {
                depth_stencil_state.depth_compare = CompareFunction::LessEqual;
                depth_stencil_state.depth_write_enabled = false;
                depth_stencil_state
            });
        Ok(())
    }
}
