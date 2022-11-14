#![cfg_attr(docsrs, feature(doc_cfg))]
//#![warn(clippy::all)]
#![warn(missing_docs)]

//!
//! A set of common assets that are useful when doing graphics, for example [TriMesh], [Texture2D] or [PbrMaterial].
//! These assets can be loaded using the [io] module or constructed manually.
//! When in memory, the assets can be for example be
//! - visualised, for example using the [three-d](https://github.com/asny/three-d) crate or in a CPU ray tracer
//! - imported into a rust-based game engine
//! - edited and saved again
//!

pub mod prelude;

mod camera;
pub use camera::*;

pub mod texture;
pub use texture::*;

pub mod material;
pub use material::*;

pub mod geometry;
pub use geometry::*;

pub mod volume;
pub use volume::*;

pub mod animation;
pub use animation::*;

use std::rc::Rc;

///
/// Model consisting of a set of [geometries](Model::geometries) and [materials](Model::materials).
/// The geometries might have a [material name](TriMesh::material_name) that matches a name of a material in the list of materials.
/// Also, the same material can be applied to several geometries.
///
#[derive(Clone, Debug)]
pub struct Model {
    /// Name.
    pub name: String,
    /// A list of [TriMesh]es.
    pub geometries: Vec<TriMesh>,
}

impl Model {
    pub fn materials(&self) -> Vec<Rc<PbrMaterial>> {
        let mut materials = Vec::new();
        for mat in self.geometries.iter().map(|g| g.material.clone()) {
            if let Some(mat) = mat {
                if !materials.iter().any(|m| Rc::ptr_eq(m, &mat)) {
                    materials.push(mat)
                }
            }
        }
        materials
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub animations: Vec<KeyFrames>,

    pub models: Vec<Model>,
}

impl Scene {
    pub fn materials(&self) -> Vec<Rc<PbrMaterial>> {
        let mut materials = Vec::new();
        for mat in self
            .models
            .iter()
            .flat_map(|m| m.geometries.iter())
            .map(|g| g.material.clone())
        {
            if let Some(mat) = mat {
                if !materials.iter().any(|m| Rc::ptr_eq(m, &mat)) {
                    materials.push(mat)
                }
            }
        }
        materials
    }
}

pub mod io;

/// A result for this crate.
pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;
///
/// Error from this crate.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{0} buffer length must be {1}, actual length is {2}")]
    InvalidBufferLength(String, usize, usize),
    #[error("the number of indices must be divisable by 3, actual count is {0}")]
    InvalidNumberOfIndices(usize),
    #[error("the max index {0} must be less than the number of vertices {1}")]
    InvalidIndices(usize, usize),
    #[error("the transformation matrix cannot be inverted and is therefore invalid")]
    FailedInvertingTransformationMatrix,
    #[cfg(feature = "image")]
    #[error("error while parsing an image file")]
    Image(#[from] image::ImageError),
    #[cfg(feature = "obj")]
    #[error("error while parsing an .obj file")]
    Obj(#[from] wavefront_obj::ParseError),

    #[cfg(feature = "pcd")]
    #[error("error while parsing an .pcd file")]
    Pcd(#[from] pcd_rs::anyhow::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[cfg(feature = "gltf")]
    #[error("error while parsing a .gltf file")]
    Gltf(#[from] ::gltf::Error),
    #[cfg(feature = "gltf")]
    #[error("the .gltf file contain corrupt buffer data")]
    GltfCorruptData,
    #[cfg(feature = "gltf")]
    #[error("the .gltf file contain missing buffer data")]
    GltfMissingData,
    #[error("the .vol file contain wrong data size")]
    VolCorruptData,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("error while loading the file {0}: {1}")]
    FailedLoading(String, std::io::Error),
    #[cfg(feature = "reqwest")]
    #[error("error while loading the url {0}: {1}")]
    FailedLoadingUrl(String, reqwest::Error),
    #[cfg(feature = "reqwest")]
    #[error("error while parsing the url {0}")]
    FailedParsingUrl(String),
    #[cfg(feature = "data-url")]
    #[error("error while parsing data-url {0}: {1}")]
    FailedParsingDataUrl(String, String),
    #[error("tried to use {0} which was not loaded or otherwise added to the raw assets")]
    NotLoaded(String),
    #[error("the feature {0} is needed")]
    FeatureMissing(String),
    #[error("failed to deserialize the file {0}")]
    FailedDeserialize(String),
    #[error("failed to serialize the file {0}")]
    FailedSerialize(String),
}
