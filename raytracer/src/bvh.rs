use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use rand::{thread_rng, Rng};
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

pub fn box_compare(
    a: &Arc<dyn Hittable>,
    b: &Arc<dyn Hittable>,
    axis_index: usize,
) -> std::cmp::Ordering {
    a.bounding_box()
        .axis_interval(axis_index as u32)
        .min
        .partial_cmp(&b.bounding_box().axis_interval(axis_index as u32).min)
        .unwrap()
}

pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl BvhNode {
    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = thread_rng().gen_range(0..3);
        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: objects[start].bounding_box(),
            }
        } else if object_span == 2 {
            Self {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bbox: AABB::two_aabb(
                    &objects[start].bounding_box(),
                    &objects[start + 1].bounding_box(),
                ),
            }
        } else {
            if axis == 0 {
                objects[start..end - 1].sort_unstable_by(|a, b| box_x_compare(a, b))
            } else if axis == 1 {
                objects[start..end - 1].sort_unstable_by(|a, b| box_y_compare(a, b))
            } else {
                objects[start..end - 1].sort_unstable_by(|a, b| box_z_compare(a, b))
            };
            objects[start..end].sort_by(|a, b| box_compare(a, b, axis));
            let mid = start + object_span / 2;
            let left = BvhNode::new(objects, start, mid);
            let right = BvhNode::new(objects, mid, end);
            Self {
                left: Arc::new(left.clone()),
                right: Arc::new(right.clone()),
                bbox: AABB::two_aabb(&left.bounding_box(), &right.bounding_box()),
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t.clone()) {
            return None;
        };
        if let Some(hit_left) = self.left.hit(r, ray_t.clone()) {
            if let Some(hit_right) = self.right.hit(r, ray_t) {
                if hit_left.t < hit_right.t {
                    Some(hit_left)
                } else {
                    Some(hit_right)
                }
            } else {
                Some(hit_left)
            }
        } else {
            self.right.hit(r, ray_t)
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
