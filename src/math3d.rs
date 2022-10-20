use std::ops::*;


const ZERO_THRESHOLD : f32 = 1e-5;

fn approx_zero<T : Normed>(x : T) -> bool {
	x.quadrance() < ZERO_THRESHOLD
}

fn close_to<T : Normed + Sub<Output = T>>(x : T, y : T) -> bool {
	approx_zero(x - y)
}

pub trait Normed {
    fn quadrance(&self) -> f32;
    fn norm(&self) -> f32 {
    	self.quadrance().sqrt()
    }
}

impl Normed for f32 {
    fn quadrance(&self) -> f32 { self * self }
    fn norm(&self) -> f32 { self.abs() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point<const N: usize> (pub [f32; N]);



pub type V2 = Point<2>;
pub type V3 = Point<3>;
pub type V4 = Point<4>;

// -- 3D SPECIFICS

impl Point<2> {
	pub const E_X : Point<2> = Self([1., 0.]);
	pub const E_Y : Point<2> = Self([0., 1.]);

	pub fn angle(angle : f32) -> Self {
		let (y, x) = angle.sin_cos();
		Self([x, y])
	}

}

impl Point<3> {
	pub const E_X : Point<3> = Self([1., 0., 0.]);
	pub const E_Y : Point<3> = Self([0., 1., 0.]);
	pub const E_Z : Point<3> = Self([0., 0., 1.]);

	pub fn vec(&self, other : &Self) -> Self {
		Self (
			[ self.0[1] * other.0[2] - self.0[2] * other.0[1]
			, self.0[2] * other.0[0] - self.0[0] * other.0[2]
			, self.0[0] * other.0[1] - self.0[1] * other.0[0] ]
		)
	}

	pub fn homo(&self) -> Point<4> {
		Point::<4> ([self.0[0], self.0[1], self.0[2], 1.])
	}
}

// -- GENERICS

impl<const N: usize> Point<N> {
	pub const ZERO : Self = Self ([0.; N]);

	pub fn new(coords : [f32; N]) -> Self {
		Self(coords)
	}

	pub fn from(from_coords : &[f32]) -> Self {
		let mut coords = [0.; N];

		for i in 0 .. N {
			coords[i] = from_coords[i];
		}

		Self(coords)
	}


	pub fn dot(&self, other: &Self) -> f32 {
		self.into_iter()
			 .zip(other.into_iter())
			 .map(|(x1, x2)| x1 * x2)
			 .sum()
	}

	pub fn scale(mut self, scale : f32) -> Self {
		for i in 0 .. N {
			self.0[i] *= scale
		}
		self
	}


	pub fn normalize(self) -> Self {
		self.scale(self.norm().recip())
	}

	pub fn from_le_bytes(bytes : &[u8]) -> Self {
		let mut to_return = [0.; N];

		let iterator =
			bytes
				.chunks(std::mem::size_of::<f32>())
				.map(|s| f32::from_le_bytes(s.try_into().unwrap()));
		for (i, x) in iterator.enumerate().take(N) {
			to_return[i] = x;
		}

		Self(to_return)
	}

	pub fn from_be_bytes(bytes : &[u8]) -> Self {
		let mut to_return = [0.; N];

		let iterator =
			bytes
				.chunks(std::mem::size_of::<f32>())
				.map(|s| f32::from_be_bytes(s.try_into().unwrap()));
		for (i, x) in iterator.enumerate().take(N) {
			to_return[i] = x;
		}

		Self(to_return)
	}
}

impl<const N : usize> Normed for Point<N> {
	fn quadrance(&self) -> f32 {
		self.into_iter()
			 .map(|x| x * x)
			 .sum()
	}
}

impl<const N: usize> IntoIterator for Point<N> {
	type Item     = f32;
	type IntoIter = <[f32; N] as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter { 
		self.0.into_iter() 
	}
}

impl<'a, const N: usize> IntoIterator for &'a Point<N> {
	type Item     = <&'a [f32; N] as IntoIterator>::Item;
	type IntoIter = <&'a [f32; N] as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter { 
		(&(self.0)).into_iter() 
	}
}

impl<const N : usize> Add for Point<N> {
	type Output = Point<N>;

	fn add(mut self, other : Self) -> Self {
		for i in 0 .. N {
			self.0[i] += other.0[i]
		}
		Self(self.0)
	}
}

impl<const N : usize> AddAssign for Point<N> {
	fn add_assign(&mut self, other: Self) { 
		for i in 0 .. N {
			self.0[i] += other.0[i]
		}
	}
}

impl<const N : usize> SubAssign for Point<N> {
	fn sub_assign(&mut self, other: Self) { 
		for i in 0 .. N {
			self.0[i] -= other.0[i]
		}
	}
}

impl<const N : usize> Sub for Point<N> {
	type Output = Point<N>;

	fn sub(mut self, other : Self) -> Self {
		for i in 0 .. N {
			self.0[i] -= other.0[i]
		}
		Self(self.0)
	}
}

/// Component by component multiplication
impl<const N : usize> Mul for Point<N> {
	type Output = Point<N>;

	fn mul(mut self, other : Self) -> Self {
		for i in 0 .. N {
			self.0[i] *= other.0[i]
		}
		Self(self.0)
	}
}

impl<const N : usize> Neg for Point<N> {
	type Output = Point<N>;

	fn neg(mut self) -> Self {
		for i in 0 .. N {
			self.0[i] = - self.0[i]
		}
		self
	}
}

impl<const N : usize> PartialEq for Point<N> {
	fn eq(&self, other: &Self) -> bool { 
		self.0 == other.0
	}
}

impl<const N : usize> Eq for Point<N> {}

// -- QUATERNIONS
// Unit quaternions
#[derive(Debug, Clone, Copy)]
pub struct Quaternion(V4);

impl Quaternion {
	pub fn new(values : V4) -> Self {
		Self(values)
	}

	pub fn rotation(axis : V3, angle : f32) -> Self {
		let axis = axis.normalize();
		let (sin, cos) = (0.5 * angle).sin_cos();
		Self(V4::new(
			[cos, axis.0[0] * sin, axis.0[1] * sin, axis.0[2] * sin,]
		))
	}

	pub fn axis_angle(&self) -> (V3, f32) {
		let unnormed_axis = V3::from(&self.0.0[1..]);
		let norm = unnormed_axis.norm();
		let axis = unnormed_axis.scale(norm.recip());
		let angle = 2. * norm.atan2(self.0.0[0]);

		(axis, angle)
	}

	pub fn to_matrix(self) -> M33 {
		let coeffs = self.0.0;
		Matrix::<3>([
			[2. * (coeffs[0] * coeffs[0] + coeffs[1] * coeffs[1]) - 1. , 2. * (coeffs[1] * coeffs[2] - coeffs[0] * coeffs[3])      , 2. * (coeffs[1] * coeffs[3] + coeffs[0] * coeffs[2])      ,],
			[2. * (coeffs[1] * coeffs[2] + coeffs[0] * coeffs[3])      , 2. * (coeffs[0] * coeffs[0] + coeffs[2] * coeffs[2]) - 1. , 2. * (coeffs[2] * coeffs[3] - coeffs[0] * coeffs[1])      ,],
			[2. * (coeffs[1] * coeffs[3] - coeffs[0] * coeffs[2])      , 2. * (coeffs[2] * coeffs[3] + coeffs[0] * coeffs[1])      , 2. * (coeffs[0] * coeffs[0] + coeffs[3] * coeffs[3]) - 1. ,],
		])
	}

	pub fn interpolate(q1 : Self, q2 : Self, t : f32) -> Self {
		let v1 = q1.0.clone();
		let v2 = q2.0.clone();
		let cos_angle = (v1.dot(&v2) / (v1.norm() * v2.norm())).abs();
		let angle = cos_angle.acos();

		let sin_angle = angle.sin();

		// If sin angle is not small, use it
		// otherwise, do a 1st order approximation
		if sin_angle > ZERO_THRESHOLD {
			let sin_t_angle      = (t * angle).sin();
			let sin_comp_t_angle = ((1. - t) * angle).sin();

			let v1 = q1.0;
			let v2 = q2.0;
			Self((v1.scale(sin_comp_t_angle) + v2.scale(sin_t_angle)).scale(sin_angle.recip()))
		}
		else {
			let v1 = q1.0;
			let v2 = q2.0;
			Self(v1.scale(t) + v2.scale(1. - t))
		}
	}



	pub const ID : Self = Self(Point::<4>([1.0, 0.0, 0.0, 0.0]));
}

impl Normed for Quaternion {
    fn quadrance(&self) -> f32 {
        self.0.quadrance()
    }
}

impl Add for Quaternion {
	type Output = Quaternion;

	fn add(mut self, other : Self) -> Self {
		Self(self.0 + other.0)
	}
}

impl AddAssign for Quaternion {
	fn add_assign(&mut self, other: Self) { 
		self.0 += other.0;
	}
}

impl SubAssign for Quaternion {
	fn sub_assign(&mut self, other: Self) { 
		self.0 -= other.0;
	}
}

impl Sub for Quaternion {
	type Output = Quaternion;

	fn sub(mut self, other : Self) -> Self {
		Self(self.0 - other.0)
	}
}

impl Neg for Quaternion {
	type Output = Quaternion;

	fn neg(mut self) -> Self {
		Self(- self.0)
	}
}

impl PartialEq for Quaternion {
	fn eq(&self, other: &Self) -> bool { 
		self.0 == other.0
	}
}

impl Eq for Quaternion {}


// q1 * q2 is equivalent to ROT2 * ROT1 is matrix settings
impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        Self(V4::new([
        	self.0.0[0] * rhs.0.0[0] - self.0.0[1] * rhs.0.0[1] - self.0.0[2] * rhs.0.0[2] - self.0.0[3] * rhs.0.0[3],
        	self.0.0[0] * rhs.0.0[1] + self.0.0[1] * rhs.0.0[0] + self.0.0[2] * rhs.0.0[3] - self.0.0[3] * rhs.0.0[2],
        	self.0.0[0] * rhs.0.0[2] - self.0.0[1] * rhs.0.0[3] + self.0.0[2] * rhs.0.0[0] + self.0.0[3] * rhs.0.0[1],
        	self.0.0[0] * rhs.0.0[3] + self.0.0[1] * rhs.0.0[2] - self.0.0[2] * rhs.0.0[1] + self.0.0[3] * rhs.0.0[0],
        ]))
    }
}


// -- MATRIX

// Convention : row-majour
// i.e. coords[i][j] is the element in the i-th row, the j-th column 
// | c00 c01 c02 |
// | c10 c11 c12 |
// | c20 c20 c22 |
// Debug shows:
// [[c00, c01, c02], [c10, c11, c12], [c20, c21, c22]]
// so matrix multiplication is expressed as: sum over k of a_ik * b_kj
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize> (pub [[f32; N]; N]);

pub type M22 = Matrix<2>;
pub type M33 = Matrix<3>;
pub type M44 = Matrix<4>;

impl M44 {
	pub fn translation(vec : V3) -> Self {
		let mut to_return = Self::id();

		for i in 0 .. 3 {
			to_return.0[i][3] = vec.0[i];
		}
		to_return
	}

	pub fn rotation(axis : &V3, angle : f32) -> Self {
		let axis = axis. normalize();
		let (sin, cos) = angle.sin_cos();
		let opcos      = 1. -  cos;
		let [x, y, z]  = axis.0;

		let cross_prod_matrix = M33::new([
			[0., -z, y],
			[z,  0., -x],
			[-y, x,  0.],
		]);

		let outer_prod_matrix = M33::new([
			[x * x, y * x, z * x],
			[x * y, y * y, z * y],
			[x * z, y * z, z * z],
		]);

		let non_homo_result = 
		      M33::id().scale(cos) +
		      cross_prod_matrix.scale(sin) +
		      outer_prod_matrix.scale(opcos);

		non_homo_result.to_homo()
	}

	pub fn scaling(scale : f32) -> Self {
		let mut to_return = M44::id().scale(scale);
		to_return.0[3][3] = 1.;

		to_return
	}

	/// Returns a matrix that takes a vector in world coordinates
	/// And returns the same vector in camera coordinates
	///
	pub fn look_at(pos_camera : &V3, target : &V3, up : &V3) -> Self {
		let mut to_return = Self::id();
		let z_camera : V3 = (pos_camera.clone() - target.clone()).normalize();
		let x_camera : V3 = up.vec(&z_camera).normalize();
		let y_camera : V3 = z_camera.vec(&x_camera).normalize();

		for i in 0 .. 3 {
			to_return.0[0][i] = x_camera.0[i];
			to_return.0[1][i] = y_camera.0[i];
			to_return.0[2][i] = z_camera.0[i];
		}

		to_return.0[0][3] = -x_camera.dot(&pos_camera);
		to_return.0[1][3] = -y_camera.dot(&pos_camera);
		to_return.0[2][3] = -z_camera.dot(&pos_camera);

		to_return
	} 

	/// Returns a matrix that takes a vector in view coordinates
	/// And returns the same vector in screen coordinates (unnormalized)
	///
	pub fn perspective_projection(clip_near : f32, clip_far : f32, fov : f32, aspect : f32) -> Self {
		let clip_region = clip_far - clip_near;
		let tan_fov     = ((180. - fov).to_radians() / 2.).tan();
		Self ([
			[tan_fov / aspect, 0., 0., 0.],
			[0., tan_fov, 0., 0.],
			[0., 0., -(clip_far + clip_near) / clip_region, -2. * clip_near * clip_far / clip_region],
			[0., 0., -1., 0.],
		])
	} 

	pub fn apply_homo(&self, v : &V3) -> V3 {
		let mut coords = [0.; 3];
		for i in 0 .. 3 {
			coords[i] = 
				v.0[0] * self.0[i][0] + 
				v.0[1] * self.0[i][1] + 
				v.0[2] * self.0[i][2] + 
				self.0[i][3]  
		}
		V3::new(coords)
	} 

	pub fn inv_ortho_homo(&self) -> Self {
		let mut to_return = Self::id();

		for i in 0 .. 3 {
			for j in 0 .. 3 {
				to_return.0[i][j] = self.0[j][i];
			}
		}

		let translation = - V3::new([
			self.0[0][3], 
			self.0[1][3], 
			self.0[2][3],
		]); 
		let out = to_return.apply_homo(&translation);

		for i in 0 .. 3 {
			to_return.0[i][3] = out.0[i];
		}


		to_return
	}


	pub fn extract_rotation(&self) -> M33 {
		let mut to_return : [[f32; 3]; 3] = [[0.0; 3]; 3];
		for i in 0 .. 3 {
			for j in 0 .. 3 {
				to_return[i][j] = self.0[i][j];
			}
		}
		M33::new(to_return)
	}
}

impl M33 {
	pub fn to_homo(self) -> M44 {
		let mut to_return = M44::id();
		for i in 0 .. 3 {
			for j in 0 .. 3 {
				to_return.0[i][j] = self.0[i][j];
			}
		}
		to_return
	}
}


impl<const N : usize> Matrix<N> {

	pub fn from_row_major(input : &[f32]) -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			for j in 0 .. N {
				coords[i][j] = input[i * N + j];
			}
		}
		Self (coords)
	}

	pub fn from_col_major(input : &[f32]) -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			for j in 0 .. N {
				coords[i][j] = input[j * N + i];
			}
		}
		Self (coords)
	}

	pub fn new(coords: [[f32; N]; N]) -> Self {
		Self(coords)
	}

	pub fn apply(&self, v : &Point<N>) -> Point<N> {
		let mut coords = [0.; N];
		for i in 0 .. N {
			for j in 0 .. N {
				coords[i] += v.0[j] * self.0[i][j];
			}
		}
		Point::<N> (coords)
	} 


	pub fn scale(mut self, scale : f32) -> Self {
		for i in 0 .. N {
			for j in 0 .. N {
				self.0[i][j] *= scale
			}
		}
		self
	}


	pub fn dot(self, other : &Self) -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			for j in 0 .. N {
				for k in 0 .. N {
					coords[i][j] += self.0[i][k] * other.0[k][j]
				}
			}
		}
		Self (coords)
	}


	pub fn id() -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			coords[i][i] = 1.
		}
		Self (coords)
	}

	pub fn transpose(&self) -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			for j in 0 .. N {
				coords[i][j] = self.0[j][i]
			}
		}	
		Self (coords)
	}

	pub fn diagonal(diag : Point::<N>) -> Self {
		let mut coords : [[f32; N]; N] = [[0.; N]; N];
		for i in 0 .. N {
			coords[i][i] = diag.0[i];
		}	
		Self (coords)
	}
}

impl<const N: usize> Normed for Matrix<N> {
	fn quadrance(&self) -> f32 {
		let mut to_return = 0.;
		for i in 0 .. N {
			for j in 0 .. N {
				to_return += self.0[i][j] * self.0[i][j];
			}
		}
		to_return
	}
}

impl<const N : usize> Add for Matrix<N> {
	type Output = Matrix<N>;

	fn add(mut self, other : Self) -> Self {
		for i in 0 .. N {
			for j in 0 .. N {
				self.0[i][j] += other.0[i][j]
			}
		}
		Self(self.0)
	}
}

impl<const N : usize> Sub for Matrix<N> {
	type Output = Matrix<N>;

	fn sub(mut self, other : Self) -> Self {
		for i in 0 .. N {
			for j in 0 .. N {
				self.0[i][j] -= other.0[i][j]
			}
		}
		Self(self.0)
	}
}


// -- TESTS


#[cfg(test)]
mod tests {
	use rand::Rng;
	use super::*;

	 #[test]
	 fn vec_is_null_on_colin() {
		let v = V3::new([1., 2., 3.]);

		let res = v.vec(&v.scale(4.));
		  assert!(approx_zero(res.norm()));
	 }

	 #[test]
	 fn vec_is_orthogonal_to_operands() {
		let v = V3::new([1., 2., 3.]);
		let w = V3::new([4., 2., 3.]);

		let res = v.vec(&w);
		  assert!(approx_zero(res.dot(&v)));
		  assert!(approx_zero(res.dot(&w)));
	 }

	 #[test]
	 fn basis_vec_orthogonal() {
		  assert!(approx_zero(V3::E_X.dot(&V3::E_Y)));
		  assert!(approx_zero(V3::E_X.dot(&V3::E_Z)));
		  assert!(approx_zero(V3::E_Y.dot(&V3::E_Z)));

		  assert!(close_to(V3::E_X.dot(&V3::E_X), 1.));
		  assert!(close_to(V3::E_Y.dot(&V3::E_Y), 1.));
		  assert!(close_to(V3::E_Z.dot(&V3::E_Z), 1.));
	 }

	 #[test]
	 fn sum_works_as_expected() {
		let res = V3::E_X.clone() 
				  + V3::E_Y.clone() 
				  + V3::E_Z.clone();

		assert!(approx_zero(V3::E_X.dot(&V3::E_Y)));
		  assert!(approx_zero(V3::E_X.dot(&V3::E_Z)));
		  assert!(approx_zero(V3::E_Y.dot(&V3::E_Z)));

		  assert!(close_to(res.0[0], 1.));
		  assert!(close_to(res.0[1], 1.));
		  assert!(close_to(res.0[2], 1.));
	 }


	#[test]
	fn apply_works_as_expected() {
		// A simple tranlation
		let w : V3 = V3::new([1., -1., 2.]);
		let v : V3 = V3::new([3., 4., -2.]);
		let translate : M44 = M44::translation(w);
		println!("translate: {:?}", translate);

		let result   = translate.apply_homo(&v);
		let expected = v + w;

		println!("Obtained: {:?}", result);
		println!("Expected: {:?}", expected);
		assert!(close_to(result, expected));

		// permutation matrix
		let matrix = M44::new([
			[0., 1., 0., 0.],
			[0., 0., 1., 0.],
			[1., 0., 0., 0.],
			[0., 0., 0., 1.],
		]);

		
		let result   = matrix.apply_homo(&v);
		let expected = V3::new([4., -2., 3.]);

		assert!(close_to(result, expected));



		// scale matrix
	 }


	#[test]
	fn look_at_invariants() {
		let pos_camera = V3::new([4., 3., 2.]);
		let target     = V3::new([1., 0., 0.]);
		let view_matrix = M44::look_at(&pos_camera, &target, &V3::E_Y);
		println!("view_matrix {:?}", view_matrix);
		println!("vect {:?}", (target - pos_camera).normalize());

		// pos_camera ought to be mapped to 0
		let v = view_matrix.apply_homo(&pos_camera);
		println!("pos_camera {:?}", v);
		assert!(approx_zero(v.norm()));

		// Camera looks along negative Z axis and target is right in line
		let v = view_matrix.apply_homo(&target);
		println!("target {:?}", v);
		assert!(v.0[2] <  0.);
		assert!(approx_zero(v.0[0]));
		assert!(approx_zero(v.0[1]));

		// "up" is truly up
		let v = view_matrix.apply_homo(&V3::E_Y);
		println!("e_y {:?}", v);
		assert!(v.0[1] >  0.);

		// right is right
		// let v = view_matrix.apply_homo(&(target + V3::E_Z))
	}

	fn norm_screen(mut vec: Point::<4>) -> Point::<4> {
		vec.0[0] /= vec.0[3];
		vec.0[1] /= vec.0[3];
		vec.0[2] /= vec.0[3];
		vec.0[3] /= vec.0[3];
		vec
	}

	#[test]
	fn rotation_test() {
		let axis : V3 = V3::E_Y;
		let angle = 90.0_f32.to_radians();

		let rot_matrix = M44::rotation(&axis, angle);  
		let v = V3::E_X;
		let result = rot_matrix.apply_homo(&v);
		let expected = -V3::E_Z;

		assert!(close_to(expected, result));

		let v = V3::E_Y;
		let result = rot_matrix.apply_homo(&v);

		assert!(close_to(v, result));
	}


	#[test]
	fn perspective_invariants() {
		let clip_near = 1.;
		let clip_far  = 10.;
		let fov       = 90.;
		let aspect    = 4. / 3.;
		let projection_matrix = M44::perspective_projection(
			clip_near, clip_far,
			fov, aspect,
		);
		println!("projection_matrix {:?}", projection_matrix);


		// Something on the clip near plane ought to have Z coordinate null
		let v = V3::new([0., 0., - clip_near]).homo();
		let result = norm_screen(projection_matrix.apply(&v));
		let expected = Point::<4>::new([0., 0., -1., 1.]);
		assert!(close_to(expected, result));

		// Something on the clip far plane ought to have Z coordinate 1
		let v = V3::new([0., 0., - clip_far]).homo();
		let result = norm_screen(projection_matrix.apply(&v));
		let expected = Point::<4>::new([0., 0., 1., 1.]);
		assert!(close_to(expected, result));

		// when fov = 90°, the tallest I can see is equal to the distance from the eye.
		let v = V3::new([0., 5., - 5.]).homo();
		let result = norm_screen(projection_matrix.apply(&v));
		println!("tallest {:?}", result);
		assert!(close_to(result.0[1], 1.));

		// when fov = 90°, the leftmost I can see is equal to the distance from the eye times times aspect ratio.
		let v = V3::new([-6. * aspect, 0., - 6.]).homo();
		let result = norm_screen(projection_matrix.apply(&v));
		println!("leftmost {:?}", result);
		assert!(close_to(result.0[0], -1.));

	 }

	#[test]
	fn inv_ortho_homo_invariants() {
		let axis  = V3::new([1., -2., 3.]);
		let angle = 2.;

		let matrix = M44::rotation(&axis, angle).dot(&M44::translation(V3::E_X));
		let expected_inv_matrix = M44::translation(-V3::E_X).dot(&M44::rotation(&axis, -angle));


		let predict_inv_matrix = matrix.inv_ortho_homo();
		assert!(close_to(expected_inv_matrix, predict_inv_matrix));
	}


	#[test]
	fn quaternion_mul() {
		let q1 = Quaternion::rotation(V3::E_X, 25_f32.to_radians());
		let q2 = Quaternion::rotation(V3::E_X, 45_f32.to_radians());
		let q3 = Quaternion::rotation(V3::E_X, 70_f32.to_radians());

		assert!(close_to(q3, q2 * q1));

		let q1 = Quaternion::rotation(V3::E_X, -90_f32.to_radians());
		let q2 = Quaternion::rotation(V3::E_Z, 90_f32.to_radians());
		let q3 = Quaternion::rotation(V3::E_X, 90_f32.to_radians());
		let q4 = Quaternion::rotation(V3::E_Y, 90_f32.to_radians());

		assert!(close_to(q4, q1 * (q2 * q3)));
	}

	#[test]
	fn from_bytes() {
		let array : [u8; 4 * 4] = [
			0x40, 0x49, 0x0f, 0xdb, // pi      big endian 32bits
			0x3f, 0x80, 0x00, 0x00, // 1       big endian 32bits
			0x37, 0xfb, 0xa8, 0x82, // 3.10-5  big endian 32bits
			0xc0, 0x80, 0x00, 0x00, // -4      big endian 32bits
		];

		assert_eq!(
			V4::new([3.1415926535897932384626433832795, 1.0, 3.0e-5, -4.]),
			V4::from_be_bytes(&array),
		);

		let array : [u8; 4 * 4] = [
			0xdb, 0x0f, 0x49, 0x40, // pi      little endian 32bits
			0x00, 0x00, 0x80, 0x3f, // 1       little endian 32bits
			0x82, 0xa8, 0xfb, 0x37, // 3.10-5  little endian 32bits
			0x00, 0x00, 0x80, 0xc0, // -4      little endian 32bits
		];

		assert_eq!(
			V4::new([3.1415926535897932384626433832795, 1.0, 3.0e-5, -4.]),
			V4::from_le_bytes(&array),
		)
	}

	#[test]
	fn quaternion_to_mat() {
		let q1 = Quaternion::rotation(V3::E_X, -90_f32.to_radians());
		let matrix = M33::new([
			[1., 0., 0.,],
			[0., 0., 1.,],
			[0.,-1., 0.,],
		]);

		assert!(close_to(q1.to_matrix(), matrix));

		let q1 = Quaternion::rotation(V3::new([1.; 3]), 120_f32.to_radians());
		let matrix = M33::new([
			[0., 0., 1.,],
			[1., 0., 0.,],
			[0., 1., 0.,],
		]);
		println!("{:?} {:?}", q1.clone().to_matrix(), matrix);
		assert!(close_to(q1.to_matrix(), matrix));

		// axis is a fixed point
		let mut rng = rand::thread_rng();
		for _ in 0 .. 20 {
			let axis = V3::new(rng.gen::<[f32; 3]>());
			let angle : f32 = rng.gen();

			let q = Quaternion::rotation(axis, angle);
			let matrix = q.to_matrix();

			let img_axis = matrix.apply(&axis);
			assert!(close_to(axis, img_axis));
		}
	}

	#[test]
	fn quaternion_interpolate() {
		let mut rng = rand::thread_rng();

		for _ in 0 .. 100 {
			let axis = V3::new(rng.gen::<[f32; 3]>());
			let angle : f32 = rng.gen();
			let q1 = Quaternion::rotation(axis, angle);


			// t = 0 and 1 yields the end points
			let axis = V3::new(rng.gen::<[f32; 3]>());
			let angle : f32 = rng.gen();
			let q2 = Quaternion::rotation(axis, angle);

			
			assert!(close_to(q1, Quaternion::interpolate(q1, q2, 0.)));
			assert!(close_to(q2, Quaternion::interpolate(q1, q2, 1.)));

			// if end points are equal, interpolation always yields the end point
			let t : f32 = rng.gen_range(0. .. 1.);
			let q3 = Quaternion::interpolate(q2, q2, t);
			assert!(close_to(q2, q3));

			// result remains a rotation
			let q3 = Quaternion::interpolate(q1, q2, t);
			assert!(close_to(1., q3.quadrance()));

			// interpolation does not go the "long way"
			let axis = V3::new(rng.gen::<[f32; 3]>()).normalize();
			let angle : f32 = rng.gen_range(0_f32 .. 90_f32);
			let q1 = Quaternion::rotation(axis, angle.to_radians());
			let angle : f32 = rng.gen_range(0_f32 .. 90_f32);
			let q2 = Quaternion::rotation(axis, -angle.to_radians());

			let t : f32 = rng.gen_range(0. .. 1.);
			let q3 = Quaternion::interpolate(q1, q2, t);


			let (new_axis, mut new_angle) = q3.axis_angle();

			assert!(close_to(new_axis, axis) || close_to(new_axis, -axis));
			if !close_to(new_axis, axis) {
				new_angle = - new_angle;
			}

			assert!(new_angle < 90_f32.to_radians() && new_angle > -90_f32.to_radians());

			// assert!(angle);


		}
	}

	#[test]
	fn test_axis_angle() {
		let mut rng = rand::thread_rng();

		let q = Quaternion(Point([0.98950016, -0.082994215, -0.07325356, -0.09292575]));


		for _ in 0 .. 100 {
			let axis = V3::new(rng.gen::<[f32; 3]>()).normalize();
			let angle : f32 = rng.gen();
			let q = Quaternion::rotation(axis, angle);

			let (axis1, angle1) = q.axis_angle();


			assert!(
				   (close_to(axis, axis1) && close_to(angle, angle1) )
				|| (close_to(axis, axis1.neg())&& close_to(angle, angle1.neg()) )
			);
		}
	}
}