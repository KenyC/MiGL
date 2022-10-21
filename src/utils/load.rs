use std::{fs::File, io::{BufReader, BufRead}};

use crate::math3d::{V3, V2};

#[derive(Debug)]
pub enum UnimplementedFeature {
    NonTriFace,
    WeightCoordinateOnVertex,
    TexCoordsNotEqualTo2,
    FaceWithNoTexture,
    FaceWithNoNormal,
}

#[derive(Debug)]
pub enum ParseErrorKind {
	NotSupported(UnimplementedFeature),
	IncorrectNComponents,
	ExpectedFloat,
	ExpectedInt,
	IOError(std::io::Error),

}

#[derive(Debug)]
pub struct ParseError {
	pub kind : ParseErrorKind,
	pub line : Option<usize>,
}

impl ParseError {
	pub fn new(kind: ParseErrorKind, line: Option<usize>) -> Self { Self { kind, line } }
}

impl From<std::io::Error> for ParseError {
	fn from(err: std::io::Error) -> Self {
		let kind = ParseErrorKind::IOError(err);
		Self {
			kind, 
			line : None,
		}
	}
}

#[repr(C)]
pub struct VertexNormal {
	pub vertex     : V3,
	pub normal     : V3,
	pub tex_coords : V2,
}


#[derive(Debug)]
pub struct Object {
	face_verts      : Vec<(usize, usize, usize)>,
	face_normals    : Vec<(usize, usize, usize)>,
	face_tex_coords : Vec<(usize, usize, usize)>,
	normals         : Vec<V3>,
	vertices        : Vec<V3>,
	tex_coords      : Vec<V2>,
}

impl Object {


	pub fn into_vertex_normals(self) -> Vec<VertexNormal> {
		let Object { vertices, face_verts: faces, face_normals, normals, face_tex_coords, tex_coords } = self;
		let mut to_return = Vec::with_capacity(3 * faces.len());

		for (((v1, v2, v3), (fn1, fn2, fn3)), (tex1, tex2, tex3)) in faces.into_iter().zip(face_normals).zip(face_tex_coords) {
			to_return.push(VertexNormal { vertex: vertices[v1], normal: normals[fn1], tex_coords: tex_coords[tex1] });
			to_return.push(VertexNormal { vertex: vertices[v2], normal: normals[fn2], tex_coords: tex_coords[tex2] });
			to_return.push(VertexNormal { vertex: vertices[v3], normal: normals[fn3], tex_coords: tex_coords[tex3] });
		}

		to_return
	}
}

impl Default for Object {
	fn default() -> Self { 
		Self { 
			vertices :       Vec::new(), 
			normals:         Vec::new(),
			tex_coords:      Vec::new(),
			face_verts:      Vec::new(),
			face_normals:    Vec::new(),
			face_tex_coords: Vec::new(),
		} 
	}
}

#[derive(Debug)]
pub struct ObjLoader {}

impl ObjLoader {
	pub fn new() -> Self { Self {  } }

	pub fn load(&self, path : &std::path::Path) -> Result<Object, ParseError> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		let mut object = Object::default();

		for (i, line) in reader.lines().enumerate() {
			let line = line?;

			if      Self::parse_normal_line(&mut object, &line, i)?  {}
			else if Self::parse_texture_line(&mut object, &line, i)? {}
			else if Self::parse_vertex_line(&mut object, &line, i)?  {}
			else if Self::parse_face_line(&mut object, &line, i)?    {}
		}

		Ok(object)
	}

	fn parse_vertex_line(object: &mut Object, line: &str, line_no: usize) -> Result<bool, ParseError> {
		if !line.starts_with("v ") {
			return Ok(false);
		}
		// this slicing should be kosher as "v " does take two bytes in UTF8
		let words : Vec<&str> = line[2..].split_ascii_whitespace().collect();

		let len = words.len();
		if len == 4 {
			return Err(ParseError::new(ParseErrorKind::NotSupported(UnimplementedFeature::WeightCoordinateOnVertex), Some(line_no)))
		}
		if len != 3 {
			return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
		}

		let coordinates : Result<Vec<f32>, ParseError> = words
			.into_iter()
			.map(|s|
				s.parse().map_err(|_| ParseError::new(ParseErrorKind::ExpectedFloat, Some(line_no)))
			)
			.collect();

		let coordinates = coordinates?;

		object.vertices.push(V3::from(&coordinates));

		Ok(true)
	}

	fn parse_normal_line(object: &mut Object, line: &str, line_no: usize) -> Result<bool, ParseError> {
		if !line.starts_with("vn ") {
			return Ok(false);
		}
		// this slicing should be kosher as "v " does take two bytes in UTF8
		let words : Vec<&str> = line[2..].split_ascii_whitespace().collect();

		if words.len() != 3 {
			return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
		}

		let coordinates : Result<Vec<f32>, ParseError> = words
			.into_iter()
			.map(|s|
				s.parse().map_err(|_| ParseError::new(ParseErrorKind::ExpectedFloat, Some(line_no)))
			)
			.collect();

		let coordinates = coordinates?;

		object.normals.push(V3::from(&coordinates));

		Ok(true)
	}

	fn parse_texture_line(object: &mut Object, line: &str, line_no: usize) -> Result<bool, ParseError> {
		if !line.starts_with("vt ") {
			return Ok(false);
		}
		// this slicing should be kosher as "vt" does take two bytes in UTF8
		let words : Vec<&str> = line[2..].split_ascii_whitespace().collect();

		let len = words.len();
		if len == 1 || len == 3 {
			return Err(ParseError::new(ParseErrorKind::NotSupported(UnimplementedFeature::TexCoordsNotEqualTo2), Some(line_no)));
		}
		if len != 2 {
			return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
		}


		let coordinates : Result<Vec<f32>, ParseError> = words
			.into_iter()
			.map(|s|
				s.parse().map_err(|_| ParseError::new(ParseErrorKind::ExpectedFloat, Some(line_no)))
			)
			.collect();


		object.tex_coords.push(V2::from(&coordinates?));

		Ok(true)
	}

	fn parse_face_line(object: &mut Object, line: &str, line_no: usize) -> Result<bool, ParseError> {
		if !line.starts_with("f ") {
			return Ok(false);
		}
		// this slicing should be kosher as "v " does take two bytes in UTF8
		let words : Vec<&str> = line[2..].split_ascii_whitespace().collect();

		let len = words.len();
		if len != 3 && len != 4 {
			return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
		}

		// create one face per successive triplet
		// this only works for tris and quads
		let mut indices         = Vec::with_capacity(3);
		let mut normal_indices  = Vec::with_capacity(3);
		let mut texture_indices = Vec::with_capacity(3);
		for i in 0 .. len - 2 {
			indices.clear();
			normal_indices.clear();
			texture_indices.clear();
			for v in &words[i .. i + 3] {
				let v = *v;
				let components : Vec<&str> = v.split('/').collect();

				if components.len() != 3 {
					return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
				}

				let vertex_index : usize = components[0].parse().map_err(|_| 
					ParseError::new(ParseErrorKind::ExpectedInt, Some(line_no))
				)?;

				if components[1].is_empty() {
					return Err(ParseError::new(ParseErrorKind::NotSupported(UnimplementedFeature::FaceWithNoTexture), Some(line_no)));
				}
				let texture_index : usize = components[1].parse().map_err(|_| 
					ParseError::new(ParseErrorKind::ExpectedInt, Some(line_no))
				)?;

				if components[2].is_empty() {
					return Err(ParseError::new(ParseErrorKind::NotSupported(UnimplementedFeature::FaceWithNoNormal), Some(line_no)));
				}
				let normal_index : usize = components[2].parse().map_err(|_| 
					ParseError::new(ParseErrorKind::ExpectedInt, Some(line_no))
				)?;
				indices.push(vertex_index - 1);
				texture_indices.push(texture_index - 1);
				normal_indices.push(normal_index - 1);
			}


			object.face_tex_coords.push((texture_indices[0], texture_indices[1], texture_indices[2]));
			object.face_verts.push((indices[0], indices[1], indices[2],));
			object.face_normals.push((normal_indices[0], normal_indices[1], normal_indices[2],));
		}

		Ok(true)
	}
}


#[cfg(test)]
mod test {
    use crate::{utils::load::{Object, ObjLoader}, math3d::V3};


	#[test]
	fn vertex_line_test() {
		let mut object = Object::default();
		let line = "v 0.000000 0.000000 -2.997484\n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.face_verts,    vec![]);


		let mut object = Object::default();
		let line = "v   0.000000   0.000000  -2.997484 \n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.face_verts,    vec![]);

		let mut object = Object::default();
		let line = "v   0.000000   0.000000   \n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1);
		assert!(result.is_err());

	}

	#[test]
	fn normal_line_test() {
		let mut object = Object::default();
		let line = "vn -0.3060 -0.8981 0.3158".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.normals, vec![V3::new([-0.3060, -0.8981, 0.3158])]);
		assert_eq!(object.face_verts,    vec![]);


		let mut object = Object::default();
		let line = "vn   0.000000   0.000000  -2.997484 \n".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.normals, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.face_verts,    vec![]);

		let mut object = Object::default();
		let line = "vn   0.000000   0.000000   \n".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1);
		assert!(result.is_err());

	}

	#[test]
	fn face_line_test() {
		let mut object = Object::default();
		let line = "f 6/1/1 11/2/1 7/3/1 4/4/1".to_string();
		let result = ObjLoader::parse_face_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices,        vec![]);
		assert_eq!(object.face_verts,           vec![(5, 10, 6), (10, 6, 3),]);
		assert_eq!(object.face_normals,    vec![(0, 0, 0), (0, 0, 0), ]);

	}
}