//! Actual Emergence CAD geometry rendered with its native RGB path tracer.
//! No image generation, external textures, backgrounds, or compositing.
use cad::{kits::lattice::TpmsSpec, mesh_solid, Solid};
use radiation::math::Vec3;
use std::{path::PathBuf, time::Instant};
use viz::{
    pbr::{self, AreaLight, Bsdf, Mesh, Object, Shape, World},
    Camera, Image,
};

fn softbox(centre: [f64; 3], width: f64, height: f64, radiance: [f64; 3]) -> AreaLight {
    let centre = Vec3(centre);
    let normal = (Vec3([0., 0., 0.06]) - centre).unit();
    let u = Vec3([0., 0., 1.]).cross(normal).unit() * width;
    let v = normal.cross(u.unit()) * height;
    AreaLight {
        corner: centre - u * 0.5 - v * 0.5,
        u,
        v,
        radiance,
    }
}

fn display(image: &Image) -> Image {
    Image {
        width: image.width,
        height: image.height,
        // Estimated photographic exposure with an exponential highlight shoulder.
        pixels: image
            .pixels
            .iter()
            .map(|p| p.map(|c| 1. - (-1.25 * c.max(0.)).exp()))
            .collect(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = PathBuf::from(args.first().map(String::as_str).unwrap_or("output"));
    let width: usize = args.get(1).map(|v| v.parse()).transpose()?.unwrap_or(512);
    let samples: usize = args.get(2).map(|v| v.parse()).transpose()?.unwrap_or(512);
    if !(64..=2048).contains(&width) || !(1..=4096).contains(&samples) {
        return Err("Use a width of 64–2048 and 1–4096 samples per pixel.".into());
    }
    std::fs::create_dir_all(&out)?;
    let started = Instant::now();

    // A 100 mm spherical sample of a gyroid network. These are prescribed
    // design dimensions, not the result of a manufacturing or physics solve.
    let periodic = TpmsSpec::new("gyroid", 0.052).solid();
    let solid = Solid::intersection(vec![periodic, Solid::sphere([0.; 3], 0.05)]);
    let mut geometry = mesh_solid(&solid, 0.00065)?;
    let report = geometry.report();
    eprintln!("Gyroid mesh: {report:?}");
    for vertex in &mut geometry.vertices {
        vertex[2] += 0.0505;
    }

    // Artistic estimates for a neutral, satin metal; not measured optical
    // constants for a specified alloy. Lighting is relative RGB radiance.
    let metal = Bsdf::Conductor {
        index: [1.4, 1.4, 1.4],
        extinction: [5.0, 5.0, 5.0],
        alpha: [0.15, 0.15],
    };
    let objects = vec![
        Object::new(Shape::Mesh(Mesh::new(geometry, true)), metal),
        Object::new(
            Shape::Quad {
                corner: Vec3([-20., -20., 0.]),
                u: Vec3([40., 0., 0.]),
                v: Vec3([0., 40., 0.]),
            },
            Bsdf::Diffuse {
                albedo: [0.016, 0.022, 0.035],
            },
        ),
    ];
    let lights = vec![
        softbox([-0.17, -0.11, 0.27], 0.13, 0.18, [10.0, 10.5, 11.0]),
        softbox([0.13, 0.07, 0.15], 0.065, 0.14, [3.2, 6.0, 9.0]),
        softbox([-0.20, 0.12, 0.20], 0.05, 0.12, [12.0, 6.5, 2.7]),
    ];
    let world = World::new(objects, lights, [0.08, 0.10, 0.15]);
    let camera = Camera::looking_at([0.135, -0.21, 0.145], [0., 0., 0.051], 33_f64.to_radians());
    let image = pbr::render_rows(&world, &camera, width, width, samples, 0, width);
    image.write_pfm(out.join("linear.pfm"))?;
    display(&image).write_png(out.join("unfiltered.png"), 1.)?;
    let filtered = pbr::denoise(&world, &camera, &image);
    display(&filtered).write_png(out.join("avatar.png"), 1.)?;
    let elapsed = started.elapsed().as_secs_f64();
    std::fs::write(out.join("settings.json"), format!(
        "{{\"width\":{width},\"samples_per_pixel\":{samples},\"seconds\":{elapsed:.3},\"mesh_spacing_m\":0.00065,\"image_generation\":false}}\n"
    ))?;
    eprintln!("Saved {} ({} spp) in {elapsed:.1}s", out.display(), samples);
    Ok(())
}
