use super::*;

pub fn prepare_major_light(major_light: &AddMajorLight) -> (LightDir, SetCamera) {
    let dir = -normalize(&glm::cross(&major_light.xdir, &major_light.ydir));
    let mut camera = SetCamera::new(
        major_light.pos,
        major_light.pos + dir,
        major_light.ydir,
        major_light.perspective,
        1.0,
        0.1,
        1000.0,
    );
    camera.trans();

    (
        LightDir {
            light: Light {
                pos: Vec4::new(major_light.pos.x, major_light.pos.y, major_light.pos.z, 1.0),
                color: major_light.color,
                shadow: true,
            },
            radius: 500.0,
            xdir: major_light.xdir,
            ydir: major_light.ydir,
            angle: major_light.perspective,
            round: false,
            smoothnes: 0.5,
            trans: camera.projection_get() * camera.view_get(),
        },
        camera,
    )
}
