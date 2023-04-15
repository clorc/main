use nalgebra::{base, Dyn, Dim, Const};
type DynamicMatrix = base::Matrix<i32, Dyn, Dyn, nalgebra::VecStorage<i32, Dyn, Dyn>>;
type DynamicVector = base::Matrix<i32, Dyn, Const<1>, nalgebra::VecStorage<i32, Dyn, Const<1>>>;
type Shape = (Dyn, Dyn);

// 2 dimensional convolution between two two matrices using Toeplitz Matrices
// algorithm source: https://assets.researchsquare.com/files/rs-2195496/v1_covered.pdf?c=1666848517
// output: (m-p+1) x (n-q+1) matrix
pub fn convolve_2d(image: DynamicMatrix, kernel: DynamicMatrix){
    let image_shape: Shape = (Dyn(image.nrows()), Dyn(image.ncols())); 
    let kernel_shape: Shape = (Dyn(kernel.nrows()), Dyn(kernel.ncols())); 

    kernel_toeplitz(kernel, &image_shape, &kernel_shape);
}

// computing the toeplitz of a pxq kernel matrix for a mxn image matrix
// output: np x (n-q+1) toeplitz matrix
fn kernel_toeplitz(kernel: DynamicMatrix, image_shape: &Shape, kernel_shape: &Shape) -> DynamicMatrix {
    // initializing the toeplitz with correct dimensions
    let new_dimensions: Shape = (Dyn(image_shape.1.value() * kernel_shape.0.value()), Dyn(image_shape.1.value() - kernel_shape.1.value() + 1));
    let mut toeplitz: DynamicMatrix = base::Matrix::zeros_generic(new_dimensions.0, new_dimensions.1);

    // padding out the kernel with zeros
    let mut extended_kernel: DynamicMatrix = kernel;
    extended_kernel = extended_kernel.insert_rows(kernel_shape.0.value(), image_shape.1.value() - kernel_shape.1.value(), 0);

    // initializing the column that we will be sliding down for our toeplitz
    let mut sliding_column: DynamicVector = base::Matrix::zeros_generic(Dyn(image_shape.1.value()*kernel_shape.0.value()), Const);
    let mut index = 0;

    // turning extended_kernel into a vector
    for element in sliding_column.iter_mut() {
        let curr = extended_kernel.get_mut(index).unwrap();
        *element = *curr;
        index+=1;
    }
    
    //computing the final toeplitz
    for i in 0..new_dimensions.1.value() {
        toeplitz.set_column(i, &sliding_column);
        
        // sliding down the entries of the sliding column
        sliding_column = sliding_column.insert_row(0, 0);
        sliding_column = sliding_column.remove_row(new_dimensions.0.value());
    }

    toeplitz
}

// computing a matrix whose columns consist of certain rows of the image that were flattened.
// output: (m-p+1) x (n-q+1) matrix
fn _flattened_row_matrix(_image: DynamicMatrix, _kernel_shape: Shape) {

}

