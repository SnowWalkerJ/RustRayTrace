use RayTrace::*;


fn main() {
    let mut scene = Scene::new();
    // sphere1
    scene.add_object(
        Sphere::new(Point::new(0.5, 0.0, 5.0), 0.6, Direction::new(0.0, 0.0, -1.0), Direction::new(-1.0, 0.0, 0.0)),
        SolidTexture::new(Color::yellow()),
        LambertianDiffuse::new(),
    );
    // sphere2
    scene.add_object(
        Sphere::new(Point::new(-1.0, -0.3, 6.0), 1.2, Direction::new(0.0, 0.0, -1.0), Direction::new(-1.0, 0.0, 0.0)),
        SolidTexture::new(Color::new(0.9, 0.9, 0.9)),
        MetalMaterial::new(),
    );
    // back
    scene.add_object(
        Quard::new(Point::new(-5.0, -5.0, 10.0), Direction::new(10.0, 0.0, 0.0), Direction::new(0.0, 10.0, 0.0)),
        SolidTexture::new(Color::white()),
        LambertianDiffuse::new(),
    );
    // left
    scene.add_object(
        Quard::new(Point::new(-5.0, -5.0, 0.0), Direction::new(0.0, 10.0, 0.0), Direction::new(0.0, 0.0, 10.0)),
        SolidTexture::new(Color::red()),
        LambertianDiffuse::new(),
    );
    // right
    scene.add_object(
        Quard::new(Point::new(5.0, -5.0, 0.0), Direction::new(0.0, 10.0, 0.0), Direction::new(0.0, 0.0, 10.0)),
        SolidTexture::new(Color::green()),
        LambertianDiffuse::new(),
    );
    // ceiling
    scene.add_object(
        Quard::new(Point::new(-5.0, 5.0, 0.0), Direction::new(10.0, 0.0, 0.0), Direction::new(0.0, 0.0, 10.0)),
        SolidTexture::new(Color::white()),
        MixtureMaterial::new(0.05, LambertianDiffuse::new(), Light::from(Color::new(0.4, 0.7, 1.0))),
    );
    // floor
    scene.add_object(
        Quard::new(Point::new(-5.0, -5.0, 0.0), Direction::new(10.0, 0.0, 0.0), Direction::new(0.0, 0.0, 10.0)),
        CheckerTexture::new(SolidTexture::from(Color::white()), SolidTexture::from(Color::black()), 0.1, 0.1, 0.0, 0.0),
        LambertianDiffuse::new(),
    );

    let camera = Camera::new(120.0, Point::new(0.0, 0.0, -2.0), Point::new(0.0, 0.0, 5.0), Direction::new(1.0, 0.0, 0.0), 0.01);
    let renderer = Renderer::new(1600, 1200, 500, 30, Color::new(0.1, 0.1, 0.1), 0.01);
    let canvas = renderer.render(&camera, &scene);
    write_ppm_to_file(&canvas, "test.ppm");
}