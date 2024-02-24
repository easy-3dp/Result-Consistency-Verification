use cgmath::Vector3;

const PI: f64 = std::f64::consts::PI;

pub fn CreatSphere(radius:f64, stacks:u32, slices:u32) -> (Vec<Vector3<f64>>, Vec<Vector3<u32>>, Vec<Vector3<f64>>) {

    let mut vertices = Vec::with_capacity(1806);
    let mut indices  = Vec::with_capacity(3600);
    let mut normals  = Vec::with_capacity(1806);

    // keeps track of the index of the next vertex that we create.
    let mut index = 0;

    /*
     First of all, we create all the faces that are NOT adjacent to the
     bottom(0,-R,0) and top(0,+R,0) vertices of the sphere.

     (it's easier this way, because for the bottom and top vertices, we need to add triangle faces.
     But for the faces between, we need to add quad faces. )
     */

    // loop through the stacks.
    for i in 1..stacks {
        let u = i as f64 / stacks as f64;
        let phi = u * PI;

        let stackBaseIndex = indices.len() >> 1;

        // loop through the slices.
        for j in 0..slices {
            let v = j as f64 / slices as f64;
            let theta = v * (PI * 2f64);

            let R = radius;
            // use spherical coordinates to calculate the positions.
            let x = theta.cos() * phi.sin();
            let y = phi.cos();
            let z = theta.sin() * phi.sin();

            vertices.push(Vector3::new(R * x, R * y, R * z));
            normals .push(Vector3::new(x, y, z));

            if i + 1 != stacks {
                // for the last stack, we don't need to add faces.

                let i1: u32; let i2: u32; let i3: u32; let i4: u32;

                if j + 1 == slices {
                    // for the last vertex in the slice, we need to wrap around to create the face.
                    i1 = index;
                    i2 = stackBaseIndex as u32;
                    i3 = index + slices;
                    i4 = stackBaseIndex as u32 + slices;
                } else {
                    // use the indices from the current slice, and indices from the next slice, to create the face.
                    i1 = index;
                    i2 = index + 1;
                    i3 = index + slices;
                    i4 = index + slices + 1;
                }

                // add quad face
                indices.push(Vector3::new(i1, i2, i3));
                indices.push(Vector3::new(i4, i3, i2));
            }

            index=index+1;
        }
    }

    /*
     Next, we finish the sphere by adding the faces that are adjacent to the top and bottom vertices.
     */

    let topIndex = index; index=index+1;
    vertices.push(Vector3::new(0.0, radius, 0.0));
    normals .push(Vector3::new(0.0, 1.0, 0.0));

    let bottomIndex = index; //index=index+1;
    vertices.push(Vector3::new(0.0, -radius, 0.0));
    normals .push(Vector3::new(0.0, -1.0, 0.0));

    for i in 0..slices {
        let mut i1 = topIndex;
        let mut i2 = i + 0;
        let mut i3 = (i + 1) % slices;
        indices.push(Vector3::new(i3, i2, i1));

        i1 = bottomIndex;
        i2 = bottomIndex - 1 - slices + (i + 0);
        i3 = bottomIndex - 1 - slices + ((i + 1) % slices);
        indices.push(Vector3::new(i1, i2, i3));
    }

    return (vertices, indices, normals);
}
