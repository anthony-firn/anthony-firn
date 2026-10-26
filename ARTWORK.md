# Profile artwork

`assets/avatar.png` is an actual render of procedural CAD geometry from
[Emergence](https://github.com/anthony-firn/emergence). No generative image
model, AI textures, stock images or composited backgrounds are used.

The subject is a 100 mm spherical sample of a gyroid network, with a 52 mm
period. Emergence's CAD kernel intersects the implicit gyroid with a sphere
and tessellates the result. Its native Rust RGB path tracer renders the
mesh, a satin metal material, three rectangular studio lights and a dark
ground plane. Geometry and material settings are prescribed for this scene;
the optical parameters are artistic estimates.

The 512 × 512 image uses 1,024 samples per pixel, deterministic sample seeds,
up to 24 path interactions, the renderer's conventional geometry-guided noise
filter, an exposure curve and sRGB encoding. There is no generative editing.
The square composition leaves room for GitHub's circular avatar crop.

## Reproduce

The complete scene is in [`render/src/main.rs`](render/src/main.rs).
It uses Emergence revision `7b515b2cedf2e50a6b42f876459a0e3d22f5eeef` and the
dependencies in `render/Cargo.lock`.

Check out `anthony-firn/emergence` alongside this repository, then run from
the profile repository root:

```sh
cargo run --locked --release --manifest-path render/Cargo.toml -- render/output/final 512 1024
cp render/output/final/avatar.png assets/avatar.png
```

The renderer also saves the unfiltered PNG, a linear-light PFM and run
settings in the ignored output directory. Smaller previews can use `256 32`
as the final two arguments.

## Project screenshots

`assets/emergence-cad.webp` shows the live Emergence CAD interface.
`assets/emberwake.webp` is a capture of the browser game.
These are application screenshots.

`assets/emergence-drawings.webp` shows the CAD prototype's drawing output.
`assets/fairground.webp` is a capture of the live Fairground app after
dismissing its introductory topic-selection screen, without posting or
signing in.

## README headers

`assets/profile-header.png` and `assets/profile-header-mobile.png` reuse
the native avatar render above. The rendered subject stays intact. A Pillow
script extends its edge colors behind the typography and positions the
render for wide and narrow screens. No generative image model is used.

Regenerate both headers from the profile repository root:

```sh
python render/profile_header.py
```

The script requires Pillow, the system Lato Regular/Heavy fonts and DejaVu
Sans Mono. On Debian or Ubuntu, the font packages are `fonts-lato` and
`fonts-dejavu-core`. The README uses a `<picture>` source to choose the
mobile composition at viewport widths up to 600 pixels.
