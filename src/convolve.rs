use nalgebra::{base, Dyn, Dim, Const};
type DynamicMatrix = base::Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>;
type DynamicVector = base::Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>>;
type Shape = (Dyn, Dyn);

// 2 dimensional convolution between two two matrices using Toeplitz Matrices
// algorithm source: https://assets.researchsquare.com/files/rs-2195496/v1_covered.pdf?c=1666848517
// output: (m-p+1) x (n-q+1) matrix
pub fn convolve_2d(image: DynamicMatrix, kernel: DynamicMatrix){

}

// computing the toeplitz of a pxq kernel matrix for a mxn image matrix
// output: np x (n-q+1) toeplitz matrix
fn kernel_toeplitz(kernel: DynamicMatrix, image_shape: Shape) {
    
}

// computing a matrix whose columns consist of certain rows of the image that were flattened.
// output: (m-p+1) x (n-q+1) matrix
fn flattened_row_matrix(image: DynamicMatrix, kernel_shape: Shape) {
    
}

