use crate::log_print;
use crate::types::*;
use crate::hittable::*;
use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::utils::*;

use std::cmp::Ordering;
use std::sync::Arc;

// --------------------------------------------------------------------------------------------------------------------

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub bbox: Aabb
}

impl BvhNode {
    pub fn build_bvh(hittables: &HittableList, time0: Float, time1: Float) -> Arc<dyn Hittable> {
        BvhNode::build(&hittables.list, 0, hittables.list.len(), time0, time1)
    }

    fn build(src_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize, time0: Float, time1: Float) -> Arc<dyn Hittable> {
         // Choose random axis to split objects
         let axis = rand_u32() % 2;
         let compare_nodes = match axis { 
             0 => { BvhNode::box_x_compare }
             1 => { BvhNode::box_y_compare }
             _ => { BvhNode::box_z_compare }
         };

        // Now split objects
        let object_span = end - start;
        let child_nodes = match object_span {
            // One item
            1 => {
                // Same reference on left and right; this avoids
                // checking for empty leaf nodes when traversing
                let left = src_objects[start].clone();
                let right = src_objects[start].clone();

                (left, right)
            }
            // Two items
            2 => {
                let left;
                let right;
                if compare_nodes(&src_objects[start], &src_objects[start+1]) == Ordering::Less {
                    left = src_objects[start].clone();
                    right = src_objects[start+1].clone();
                } else {
                    left = src_objects[start+1].clone();
                    right = src_objects[start].clone();
                }

                (left, right)
            }
            // And more items
            _ => {
                // Sort objects
                let mut sorted_objects = src_objects.as_slice()[start..end].to_vec();
                sorted_objects.sort_by(|a, b| { compare_nodes(a, b) });

                // Split list down the middle between left and right
                let mid = start + (object_span/2);
                let left = BvhNode::build(&sorted_objects.as_slice()[start..mid].to_vec(), 0, mid - start, time0, time1);
                let right = BvhNode::build(&sorted_objects.as_slice()[mid..end].to_vec(), 0, end - mid, time0, time1);

                (left, right)
            }
        };


        let box_left = child_nodes.0.bounding_box(time0, time1);
        let box_right = child_nodes.1.bounding_box(time0, time1);

        let aabb;
        if box_left.is_none() || box_right.is_none() {
            log_print!("No bounding box in bvh_node constructor.");
            aabb = Aabb::default();
        } else {
            aabb = Aabb::surrounding_box(&box_left.unwrap(), &box_right.unwrap())
        }

        Arc::new( Self {
            left: child_nodes.0,
            right: child_nodes.1,
            bbox: aabb
        })
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        let a_bbox = a.bounding_box(0.0, 0.0);
        let b_bbox = b.bounding_box(0.0, 0.0);
        if a_bbox.is_none() || b_bbox.is_none() {
            log_print!("No bounding box in bvh_node constructor.");
            return Ordering::Equal;
        }

        let a_value = a_bbox.unwrap().min[axis];
        let b_value = b_bbox.unwrap().min[axis];
        if a_value < b_value {
            return Ordering::Less;
        } else if a_value > b_value {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
    
    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 0)
    }
    
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 1)
    }
    
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray<Float>, t_min: Float, t_max: Float) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) == false {
            return Option::None;
        }

        let left_hit = self.left.hit(&r, t_min, t_max);
        let right_tmax = if left_hit.is_none() { t_max } else { left_hit.as_ref().unwrap().t };
        let right_hit = self.right.hit(&r, t_min, right_tmax);

        if right_hit.is_some() {
            return right_hit;
        } else if left_hit.is_some() {
            return left_hit;
        } else {
            return Option::None;
        }
    }

    fn bounding_box(&self, _time0: Float, _time1: Float) -> Option<Aabb> {
        Some(self.bbox)
    }
}