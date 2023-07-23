use nalgebra::{base, Dyn, Dim};
type DynamicMatrix = base::Matrix<f32, Dyn, Dyn, nalgebra::VecStorage<f32, Dyn, Dyn>>;
type Shape = (Dyn, Dyn);

// given a m x n matrix and p x q pool, returns matrix after being max_pooled
// p divides m and q divides n
// output: (m/p) x (n/q) matrix
pub fn max_pool(image: DynamicMatrix, pool_size: Shape) -> DynamicMatrix{
    let (p, q) = pool_size;
    let m = image.nrows();
    let n = image.ncols();

    assert!((m%p.value() == 0) && (n%q.value() == 0));

    let mut max_pooled: DynamicMatrix = base::Matrix::zeros_generic(Dyn(m/p.value()), Dyn(n/q.value()));

    for row in (0..m).step_by(p.value()) {
        for col in (0..n).step_by(q.value()) {
            max_pooled[(row/p.value(),col/q.value())] = image.view((row,col), (p.value(),q.value())).max();
        }
    }

    max_pooled
}