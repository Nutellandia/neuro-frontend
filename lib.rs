extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Layout Memoria: [x, y, z, vx, vy, vz] per ogni neurone
const STRIDE: usize = 6;

#[wasm_bindgen]
pub struct NeuroPhysicsEngine {
    data: Vec<f32>,
    count: usize,
}

#[wasm_bindgen]
impl NeuroPhysicsEngine {
    pub fn new(count: usize) -> NeuroPhysicsEngine {
        let mut data = Vec::with_capacity(count * STRIDE);
        for _ in 0..count {
            // Posizioni random iniziali (-500 a 500)
            data.push((js_sys::Math::random() as f32 - 0.5) * 1000.0); // x
            data.push((js_sys::Math::random() as f32 - 0.5) * 1000.0); // y
            data.push((js_sys::Math::random() as f32 - 0.5) * 1000.0); // z
            data.push(0.0); // vx
            data.push(0.0); // vy
            data.push(0.0); // vz
        }
        NeuroPhysicsEngine { data, count }
    }

    pub fn tick(&mut self, repulsion_force: f32) {
        let count = self.count;
        
        // 1. Applica Forze
        for i in 0..count {
            let idx = i * STRIDE;
            let x = self.data[idx];
            let y = self.data[idx+1];
            let z = self.data[idx+2];

            // Attrazione al centro (Omeostasi)
            let dist_sq = x*x + y*y + z*z;
            let dist = dist_sq.sqrt();
            let homeo_force = -0.005 * dist; // Molla elastica verso 0,0,0
            
            self.data[idx+3] += (x / dist) * homeo_force;
            self.data[idx+4] += (y / dist) * homeo_force;
            self.data[idx+5] += (z / dist) * homeo_force;

            // Moto Browniano (Energia Termica)
            self.data[idx+3] += (js_sys::Math::random() as f32 - 0.5) * repulsion_force;
            self.data[idx+4] += (js_sys::Math::random() as f32 - 0.5) * repulsion_force;
            self.data[idx+5] += (js_sys::Math::random() as f32 - 0.5) * repulsion_force;

            // Attrito (Viscosità del citoplasma)
            self.data[idx+3] *= 0.92;
            self.data[idx+4] *= 0.92;
            self.data[idx+5] *= 0.92;
        }

        // 2. Integrazione Velocità (Eulero)
        for i in 0..count {
            let idx = i * STRIDE;
            self.data[idx]   += self.data[idx+3];
            self.data[idx+1] += self.data[idx+4];
            self.data[idx+2] += self.data[idx+5];
        }
    }

    // Puntatore alla memoria raw per accesso diretto JS (Zero-Copy)
    pub fn get_data_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }
}