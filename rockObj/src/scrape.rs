use std::collections::VecDeque;
use cgmath::{Vector3, InnerSpace, MetricSpace};

type Cell = Vec<u32>;

pub fn GetNeighbours(positions_len:usize, cells:&Vec<Vector3<u32>>) -> Vec<Cell> {
    /*
     adjacentVertices[i] contains a set containing all the indices of the neighbours of the vertex with
     index i.
     A set is used because it makes the algorithm more convenient.
     */
    //let adjacentVertices = [cell; positions.len()];
    let mut adjacentVertices = vec![Cell::with_capacity(16); positions_len];

    // go through all faces.
    for iCell in 0..cells.len() {
        let cellPositions = cells[iCell];
        // go through all the points of the face.
        for iPosition in 0..3 {
            // the neighbours of this points are the previous and next points(in the array)
            let cur  = cellPositions[wrap(iPosition + 0) as usize] as usize;
            let prev = cellPositions[wrap(iPosition - 1) as usize];
            let next = cellPositions[wrap(iPosition + 1) as usize];
            // create set on the fly if necessary.
            // if (adjacentVertices[cur] == null) {
            //     adjacentVertices[cur] = cell::new();
            // }
            // add adjacent vertices.
            adjacentVertices[cur].push(prev);
            adjacentVertices[cur].push(next);
        }
    }
    return adjacentVertices;
}

fn wrap(i:i32) -> i32 {
    if i < 0 {
        3 + i
    } else {
        i % 3
    }
}

/*
Projects the point `p` onto the plane defined by the normal `n` and the point `r0`
 */
fn Project(n:Vector3<f64>, r0:Vector3<f64>, p:Vector3<f64>) -> Vector3<f64>{
    // For an explanation of the math, see http://math.stackexchange.com/a/100766
    let o = r0 - p;
    let t = n.dot(o) / n.dot(n);
    p+n*t
}
// scrape at vertex with index `positionIndex`.
pub(crate) fn Main(
    positionIndex:usize,
    positions:&mut Vec<Vector3<f64>>,
    normals:&mut Vec<Vector3<f64>>,
    adjacentVertices:&Vec<Vec<u32>>,
    strength:f64, radius:f64,
    traversed:&mut Vec<bool>,
    stack:&mut VecDeque<usize>,
)
{

    let centerPosition = positions[positionIndex];
    // to scrape, we simply project all vertices that are close to `centerPosition`
    // onto a plane. The equation of this plane is given by dot(n, r-r0) = 0,
    // where n is the plane normal, r0 is a point on the plane(in our case we set this to be the projected center),
    // and r is some arbitrary point on the plane.
    let n = normals[positionIndex];
    let r0 = centerPosition + n * -strength;
    stack.push_back(positionIndex);
    /*
     We use a simple flood-fill algorithm to make sure that we scrape all vertices around the center.
     This will be fast, since all vertices have knowledge about their neighbours.
     */
    while stack.len() > 0 {
        let topIndex = stack.pop_front().unwrap();
        if traversed[topIndex] {continue;} // already traversed; look at next element in stack.
        traversed[topIndex] = true;
        // project onto plane.
        let p = positions[topIndex];
        let projectedP = Project(n, r0, p);
        if projectedP.distance2(r0) < radius {
            positions[topIndex] = projectedP;
              normals[topIndex] = n;
        } else {
            continue;
        }
        let neighbourIndices = &adjacentVertices[topIndex];
        for i in 0..neighbourIndices.len() {
            stack.push_back(neighbourIndices[i] as usize);
        }
    }
}