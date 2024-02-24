use cgmath::{Vector3, InnerSpace, Zero};
use string_builder::Builder;

pub struct BufferGeometry {
    pub indices:Vec<u32>,
    pub positions:Vec<Vector3<f64>>,
    pub normals:Vec<Vector3<f64>>,
}

impl BufferGeometry {
    pub fn new(positions:Vec<Vector3<f64>>, indices:Vec<u32>) -> Self {
        let len = positions.len();
		Self {
            indices,
            positions,
			normals: vec![Vector3::zero();len]
		}
	}

    pub fn ComputeVertexNormals(&mut self)
    {
        // indexed elements
        for i in 0..self.indices.len()/3 {
            let vA = self.indices[i*3+0] as usize;
            let vB = self.indices[i*3+1] as usize;
            let vC = self.indices[i*3+2] as usize;
            let pA = self.positions[vA];
            let pB = self.positions[vB];
            let pC = self.positions[vC];

            let cb = pC - pB;
            let ab = pA - pB;
            let cr = cb.cross(ab);

            self.normals[vA] += cr;
            self.normals[vB] += cr;
            self.normals[vC] += cr;
        }

        for i in 0..self.normals.len() {
            let mut normal = self.normals[i];
            let n = 1.0 / normal.dot(normal).sqrt();
            normal *= n;
        }

    }

    pub fn wavefront_loadobj(&mut self) -> (Vec<Vector3<f64>>, Vec<u32>) {
        let mut out_vertices = vec![Vector3::new(0.0,0.0,0.0); self.positions.len()];
        let mut out_indices           = vec![0; self.indices.len()];
        let mut tmp           		= vec![u32::MAX; self.indices.len()];
    
        let mut offset = 0u32;
        for i in 0..self.indices.len() {
            let index = self.indices[i] as usize;
            if tmp[index] == u32::MAX {
                out_vertices[offset as usize] = self.positions[index];
                tmp[index] = offset;
                offset=offset+1;
            }
            out_indices[i] = tmp[index];
        }
    
        (out_vertices, out_indices)
    }

    pub fn parse(&self) -> String {
        let mut sb = Builder::default();
        sb.append("o\n");

        for p in &self.positions {
            sb.append(format!("v {:.2} {:.2} {:.2}\n", p[0], p[1], p[2]));
        }

        for p in &self.normals {
            sb.append(format!("vn {:.4} {:.4} {:.4}\n", p[0], p[1], p[2]));
        }

        for i in (0..self.indices.len()).step_by(3) {
            sb.append(format!("f {}//{} {}//{} {}//{}\n", self.indices[i + 0] + 1,self.indices[i + 0] + 1, self.indices[i + 1] + 1,self.indices[i + 1] + 1, self.indices[i + 2] + 1, self.indices[i + 2] + 1));
        }

        sb.string().unwrap()

    }

}