use crate::bvh::BvhNode;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material, Metal};
use crate::texture::ImageTexture;
use crate::triangle::Triangle;
use crate::vec3::Point3;
use std::sync::Arc;

pub fn read_obj(obj_filename: &str, scale: f64) -> HittableList {
    let mut object = HittableList::new();

    let filename = String::from(obj_filename);

    let obj = tobj::load_obj(
        format!("objects/{}", filename),
        &tobj::LoadOptions {
            single_index: false,
            triangulate: false,
            ignore_points: true,
            ignore_lines: true,
        },
    );
    assert!(obj.is_ok());

    let (models, materials) = obj.expect("Failed to load OBJ file");
    let mut mat_is_ok = false;
    if materials.is_ok() {
        mat_is_ok = true;
    } else {
    }

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        let mut mat: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
        if mat_is_ok {
            if let Some(id) = mesh.material_id {
                if let Some(diffuse) = materials.clone().unwrap()[id].diffuse {
                    mat = Arc::new(Lambertian::new(Color::new(
                        diffuse[0] as f64,
                        diffuse[1] as f64,
                        diffuse[2] as f64,
                    )));
                } else if let Some(specular) = materials.clone().unwrap()[id].specular {
                    if let Some(shininess) = materials.clone().unwrap()[id].shininess {
                        mat = Arc::new(Metal::new(
                            Color::new(specular[0] as f64, specular[1] as f64, specular[2] as f64),
                            shininess as f64,
                        ));
                    } else {
                    }
                } else if let Some(diffuse_texture) =
                    &materials.clone().unwrap()[id].diffuse_texture
                {
                    let tex = Arc::new(ImageTexture::new(diffuse_texture));
                    mat = Arc::new(Lambertian::new_tex(tex));
                } else {
                }
            }
        }

        if mesh.face_arities.len() != 0 {
            let mut next_face = 0;
            for f in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                next_face = end;

                let mut p: [Point3; 3] = [Color::white(); 3];
                let mut t = 0;
                let p0 = *face_indices[0] as usize;
                p[0] = Point3::new(
                    mesh.positions[p0 * 3] as f64 * scale,
                    mesh.positions[p0 * 3 + 1] as f64 * scale,
                    mesh.positions[p0 * 3 + 2] as f64 * scale,
                );
                for v in face_indices {
                    t += 1;
                    p[1] = p[2];
                    p[2] = Point3::new(
                        mesh.positions[*v as usize * 3] as f64 * scale,
                        mesh.positions[*v as usize * 3 + 1] as f64 * scale,
                        mesh.positions[*v as usize * 3 + 2] as f64 * scale,
                    );

                    if t >= 3 {
                        object.add(Arc::new(Triangle::new(&p[0], &p[1], &p[2], mat.clone())));
                    }
                }
            }
        } else {
            let mut p: [Point3; 3] = [Color::white(); 3];
            let mut t = 0;
            for v in &mesh.indices {
                p[t] = Point3::new(
                    mesh.positions[*v as usize * 3] as f64 * scale,
                    mesh.positions[*v as usize * 3 + 1] as f64 * scale,
                    mesh.positions[*v as usize * 3 + 2] as f64 * scale,
                );
                t += 1;
                if t == 3 {
                    object.add(Arc::new(Triangle::new(&p[0], &p[1], &p[2], mat.clone())));
                    t = 0;
                }
            }
        }
    }
    object = HittableList::new_from(Arc::new(BvhNode::from_list(&mut object)));
    object
}
