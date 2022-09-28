use std::{fs::File, io::{BufReader, BufRead}};

use crate::math3d::V3;

#[derive(Debug)]
pub enum UnimplementedFeature {
    NonTriFace,
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
	kind : ParseErrorKind,
	line : Option<usize>,
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
	pub vertex : V3,
	pub normal : V3,
}


#[derive(Debug)]
pub struct Object {
	vertices : Vec<V3>,
	faces : Vec<(usize, usize, usize)>,
	face_normals : Vec<(usize, usize, usize)>,
	normals : Vec<V3>,
}

impl Object {
	pub fn new() -> Self { 
		Self { 
			vertices :    Vec::new(), 
			faces    :    Vec::new(),
			face_normals: Vec::new(),
			normals:      Vec::new(),
		} 
	}

	pub fn into_vertex_normals(self) -> Vec<VertexNormal> {
		let Object { vertices, faces, face_normals, normals } = self;
		let mut to_return = Vec::with_capacity(3 * faces.len());

		for ((v1, v2, v3), (fn1, fn2, fn3)) in faces.into_iter().zip(face_normals) {
			to_return.push(VertexNormal { vertex: vertices[v1], normal: normals[fn1] });
			to_return.push(VertexNormal { vertex: vertices[v2], normal: normals[fn2] });
			to_return.push(VertexNormal { vertex: vertices[v3], normal: normals[fn3] });
		}

		to_return
	}
}


#[derive(Debug)]
pub struct ObjLoader {}

impl ObjLoader {
	pub fn new() -> Self { Self {  } }

	pub fn load(&self, path : &std::path::Path) -> Result<Object, ParseError> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		let mut object = Object::new();

		for (i, line) in reader.lines().enumerate() {
			let line = line?;

			if      Self::parse_normal_line(&mut object, &line, i)? {}
			else if Self::parse_vertex_line(&mut object, &line, i)? {}
			else if Self::parse_face_line(&mut object, &line, i)? {}
		}

		Ok(object)
	}

	fn parse_vertex_line(object: &mut Object, line: &str, line_no: usize) -> Result<bool, ParseError> {
		if !line.starts_with("v ") {
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
		let mut indices = Vec::with_capacity(3);
		let mut normal_indices = Vec::with_capacity(3);
		for i in 0 .. len - 2 {
			indices.clear();
			normal_indices.clear();
			for v in &words[i .. i + 3] {
				let v = *v;
				let components : Vec<&str> = v.split('/').collect();

				if components.len() != 3 {
					return Err(ParseError::new(ParseErrorKind::IncorrectNComponents, Some(line_no)));
				}
				let vertex_index : usize = components[0].parse().map_err(|_| 
					ParseError::new(ParseErrorKind::ExpectedInt, Some(line_no))
				)?;
				let normal_index : usize = components[2].parse().map_err(|_| 
					ParseError::new(ParseErrorKind::ExpectedInt, Some(line_no))
				)?;
				indices.push(vertex_index - 1);
				normal_indices.push(normal_index - 1);
			}



			object.faces.push((indices[0], indices[1], indices[2],));
			object.face_normals.push((normal_indices[0], normal_indices[1], normal_indices[2],));
		}

		Ok(true)
	}
}


mod test {


	#[test]
	fn vertex_line_test() {
		let mut object = Object::new();
		let line = "v 0.000000 0.000000 -2.997484\n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.faces,    vec![]);


		let mut object = Object::new();
		let line = "v   0.000000   0.000000  -2.997484 \n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.faces,    vec![]);

		let mut object = Object::new();
		let line = "v   0.000000   0.000000   \n".to_string();
		let result = ObjLoader::parse_vertex_line(&mut object, &line, 1);
		assert!(result.is_err());

	}

	#[test]
	fn normal_line_test() {
		let mut object = Object::new();
		let line = "vn -0.3060 -0.8981 0.3158".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.normals, vec![V3::new([-0.3060, -0.8981, 0.3158])]);
		assert_eq!(object.faces,    vec![]);


		let mut object = Object::new();
		let line = "vn   0.000000   0.000000  -2.997484 \n".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.normals, vec![V3::new([0.0, 0.0, -2.997484])]);
		assert_eq!(object.faces,    vec![]);

		let mut object = Object::new();
		let line = "vn   0.000000   0.000000   \n".to_string();
		let result = ObjLoader::parse_normal_line(&mut object, &line, 1);
		assert!(result.is_err());

	}

	#[test]
	fn face_line_test() {
		let mut object = Object::new();
		let line = "f 6/1/1 11/2/1 7/3/1 4/4/1".to_string();
		let result = ObjLoader::parse_face_line(&mut object, &line, 1).expect("No error should occur");
		assert!(result);

		assert_eq!(object.vertices,        vec![]);
		assert_eq!(object.faces,           vec![(6, 11, 7), (11, 7, 4),]);
		assert_eq!(object.face_normals,    vec![1, 1,]);

	}
}