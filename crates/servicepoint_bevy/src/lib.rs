// Source: https://github.com/paulkre/bevy_image_export/
// License: MIT / Apache-2.0

use bevy::{
    ecs::{
        query::QueryItem,
        system::{lifetimeless::SRes, SystemParamItem},
    },
    prelude::*,
    render::{
        camera::{CameraUpdateSystem, RenderTarget, ScalingMode},
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        graph::CameraDriverLabel,
        render_asset::{
            PrepareAssetError, RenderAsset, RenderAssetPlugin,
            RenderAssetUsages, RenderAssets,
        },
        render_graph::{
            Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel,
        },
        render_resource::{
            Buffer, BufferDescriptor, BufferUsages, Extent3d, ImageCopyBuffer,
            ImageDataLayout, MapMode, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        renderer::{RenderContext, RenderDevice},
        Render, RenderApp, RenderSet,
    },
};
use futures::channel::oneshot;
use image::{ImageBuffer, Rgba};
use wgpu::Maintain;

use servicepoint::{
    BitVec, Command::BitmapLinearWin, CompressionCode::Lzma, Connection,
    DataRef, Origin, PixelGrid,
};

#[derive(Asset, Reflect, Clone, Default)]
pub struct ServicePointExportSource {
    pub image: Handle<Image>,
}

/// Settings for handling an
#[derive(Component, Clone)]
pub struct ServicePointExportSettings {
    pub origin: Origin,
    pub window_width: usize,
    pub window_height: usize,
}

pub struct GpuImageExportSource {
    pub buffer: Buffer,
    pub source_handle: Handle<Image>,
    pub source_size: Extent3d,
    pub bytes_per_row: u32,
    pub padded_bytes_per_row: u32,
}

#[derive(Bundle)]
pub struct ImageExportBundle {
    pub source: Handle<ServicePointExportSource>,
    pub settings: ServicePointExportSettings,
}

/// Plugin enabling the generation of image sequences.
pub struct ServicePointPlugin {
    pub bind: String,
}

#[derive(Resource)]
struct ServicePointPluginConnection {
    pub connection: Connection,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ImageExportSystems {
    SetupImageExport,
    SetupImageExportFlush,
}

impl RenderAsset for ServicePointExportSource {
    type PreparedAsset = GpuImageExportSource;
    type Param = (SRes<RenderDevice>, SRes<RenderAssets<Image>>);

    fn asset_usage(&self) -> RenderAssetUsages {
        RenderAssetUsages::RENDER_WORLD
    }

    fn prepare_asset(
        self,
        (device, images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self>> {
        let gpu_image = images.get(&self.image).unwrap();

        let size = gpu_image.texture.size();
        let format = &gpu_image.texture_format;
        let bytes_per_row = (size.width / format.block_dimensions().0)
            * format.block_copy_size(None).unwrap();
        let padded_bytes_per_row =
            RenderDevice::align_copy_bytes_per_row(bytes_per_row as usize)
                as u32;

        let source_size = gpu_image.texture.size();

        Ok(GpuImageExportSource {
            buffer: device.create_buffer(&BufferDescriptor {
                label: Some("Image Export Buffer"),
                size: (source_size.height * padded_bytes_per_row) as u64,
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            source_handle: self.image,
            source_size,
            bytes_per_row,
            padded_bytes_per_row,
        })
    }
}

impl ExtractComponent for ServicePointExportSettings {
    type QueryData = (&'static Self, &'static Handle<ServicePointExportSource>);
    type QueryFilter = ();
    type Out = (Self, Handle<ServicePointExportSource>);

    fn extract_component(
        (settings, source_handle): QueryItem<'_, Self::QueryData>,
    ) -> Option<Self::Out> {
        Some((settings.clone(), source_handle.clone_weak()))
    }
}

fn get_image_bytes_from_gpu(
    gpu_source: &GpuImageExportSource,
    render_device: &Res<RenderDevice>,
) -> Vec<u8> {
    let source_size = gpu_source.source_size;

    let slice = gpu_source.buffer.slice(..);

    let (mapping_tx, mapping_rx) = oneshot::channel();

    render_device.map_buffer(&slice, MapMode::Read, move |res| {
        mapping_tx.send(res).unwrap();
    });

    render_device.poll(Maintain::Wait);
    futures_lite::future::block_on(mapping_rx).unwrap().unwrap();

    let vec = slice.get_mapped_range().to_vec();
    gpu_source.buffer.unmap();

    let bytes_per_row = gpu_source.bytes_per_row as usize;
    let padded_bytes_per_row = gpu_source.padded_bytes_per_row as usize;
    if bytes_per_row == padded_bytes_per_row {
        vec
    } else {
        let mut unpadded_bytes = Vec::<u8>::with_capacity(
            source_size.height as usize * bytes_per_row,
        );

        for padded_row in vec.chunks(padded_bytes_per_row) {
            unpadded_bytes.extend_from_slice(&padded_row[..bytes_per_row]);
        }

        unpadded_bytes
    }
}

fn send_buffer_to_connection(
    export_bundles: Query<(
        &Handle<ServicePointExportSource>,
        &ServicePointExportSettings,
    )>,
    sources: Res<RenderAssets<ServicePointExportSource>>,
    render_device: Res<RenderDevice>,
    connection: Res<ServicePointPluginConnection>,
) {
    for (source_handle, settings) in &export_bundles {
        let gpu_source = match sources.get(source_handle) {
            None => continue,
            Some(source) => source,
        };

        let image_bytes = get_image_bytes_from_gpu(gpu_source, &render_device);

        let buffer = match ImageBuffer::<Rgba<u8>, _>::from_raw(
            settings.window_width as u32,
            settings.window_height as u32,
            image_bytes,
        ) {
            None => {
                println!("Failed creating image buffer");
                continue;
            }
            Some(buffer) => buffer,
        };

        let mut bit_vec =
            BitVec::new(settings.window_height * settings.window_width);

        for (index, pixel) in buffer.chunks_exact(4).enumerate() {
            assert_eq!(pixel.len(), 4);
            if pixel[0] != 0 || pixel[1] != 0 || pixel[2] != 0 {
                bit_vec.set(index, pixel[3] != 0);
            }
        }

        let pixels = PixelGrid::load(
            settings.window_width,
            settings.window_height,
            bit_vec.data_ref(),
        );

        connection
            .connection
            .send(BitmapLinearWin(settings.origin, pixels, Lzma))
            .expect("send failed");
    }
}

impl Plugin for ServicePointPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                ImageExportSystems::SetupImageExport,
                ImageExportSystems::SetupImageExportFlush,
            )
                .chain()
                .before(CameraUpdateSystem),
        )
        .register_type::<ServicePointExportSource>()
        .init_asset::<ServicePointExportSource>()
        .register_asset_reflect::<ServicePointExportSource>()
        .add_plugins((
            RenderAssetPlugin::<ServicePointExportSource>::default(),
            ExtractComponentPlugin::<ServicePointExportSettings>::default(),
        ))
        .add_systems(
            PostUpdate,
            (
                apply_deferred
                    .in_set(ImageExportSystems::SetupImageExportFlush),
            ),
        );

        let render_app = app.sub_app_mut(RenderApp);

        let connection =
            Connection::open(&self.bind).expect("could not connect to display");

        render_app
            .insert_resource(ServicePointPluginConnection { connection })
            .add_systems(
                Render,
                send_buffer_to_connection
                    .after(RenderSet::Render)
                    .before(RenderSet::Cleanup),
            );

        let mut graph =
            render_app.world.get_resource_mut::<RenderGraph>().unwrap();

        graph.add_node(ImageExportLabel, ImageExportNode);
        graph.add_node_edge(CameraDriverLabel, ImageExportLabel);
    }
}

pub fn make_export_bundle(
    settings: ServicePointExportSettings,
    scaling_mode: ScalingMode,
    main_camera: &mut bevy::ecs::system::EntityCommands,
    images: &mut ResMut<Assets<Image>>,
    export_sources: &mut ResMut<Assets<ServicePointExportSource>>,
) -> ImageExportBundle {
    // Create an output texture.
    let size = Extent3d {
        width: settings.window_width as u32,
        height: settings.window_height as u32,
        ..default()
    };
    let mut export_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    export_texture.resize(size);

    let export_texture = images.add(export_texture);

    let mut export_camera = Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(export_texture.clone()),
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    };

    export_camera.projection.scaling_mode = scaling_mode;

    // render main camera to output texture as well
    main_camera.with_children(|parent| {
        parent.spawn(export_camera);
    });

    let export_source = ServicePointExportSource {
        image: export_texture,
    };

    ImageExportBundle {
        source: export_sources.add(export_source),
        settings,
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct ImageExportLabel;

pub struct ImageExportNode;

impl Node for ImageExportNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for (_, source) in world
            .resource::<RenderAssets<ServicePointExportSource>>()
            .iter()
        {
            if let Some(gpu_image) = world
                .resource::<RenderAssets<Image>>()
                .get(&source.source_handle)
            {
                render_context.command_encoder().copy_texture_to_buffer(
                    gpu_image.texture.as_image_copy(),
                    ImageCopyBuffer {
                        buffer: &source.buffer,
                        layout: ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(source.padded_bytes_per_row),
                            rows_per_image: None,
                        },
                    },
                    source.source_size,
                );
            }
        }

        Ok(())
    }
}
