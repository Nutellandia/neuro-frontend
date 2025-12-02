extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct NeuroPhysicsEngine { data: Vec<f32> }
#[wasm_bindgen]
impl NeuroPhysicsEngine {
    pub fn new(count: usize) -> NeuroPhysicsEngine { NeuroPhysicsEngine { data: vec![0.0; count * 6] } }
    pub fn tick(&mut self, _force: f32) { /* Logic */ }
}