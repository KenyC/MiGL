use gl::types::*;


// -- LOG

#[derive(Debug)]
pub enum LogKind {
	CompilerLog,
	LinkLog,
}


pub fn get_log(id : GLuint, log_kind : LogKind) -> Option<String> {
	let mut success: gl::types::GLint = 1;
	unsafe {
		match &log_kind {
			LogKind::CompilerLog => gl::GetShaderiv  (id, gl::COMPILE_STATUS, &mut success),
			LogKind::LinkLog     => gl::GetProgramiv (id, gl::LINK_STATUS,    &mut success),
		}
	}


	if success == 0 {
		let mut info_log_length: gl::types::GLint = -1;
		unsafe {
			match &log_kind {
				LogKind::CompilerLog => gl::GetShaderiv  (id, gl::INFO_LOG_LENGTH, &mut info_log_length),
				LogKind::LinkLog     => gl::GetProgramiv (id, gl::INFO_LOG_LENGTH, &mut info_log_length),
			}
		}
		println!("info_log_length {:?}", info_log_length);
		// info_log_length = 50;

		// -- CREATE BUFFER OF CORRECT SIZE
		// allocate buffer of correct size
		let mut buffer: Vec<u8> = Vec::with_capacity(info_log_length as usize + 1);
		// fill it with info_log_length spaces
		for _ in 1 .. info_log_length {buffer.push(b' ')}
		// convert buffer to CString
		let error: std::ffi::CString = unsafe { std::ffi::CString::from_vec_unchecked(buffer) };

		unsafe {
			match &log_kind {
				LogKind::CompilerLog => gl::GetShaderInfoLog(
				    id,
				    info_log_length,
				    std::ptr::null_mut(),
				    error.as_ptr() as *mut gl::types::GLchar
				),
				LogKind::LinkLog => gl::GetProgramInfoLog(
				    id,
				    info_log_length,
				    std::ptr::null_mut(),
				    error.as_ptr() as *mut gl::types::GLchar
				),

			}
		}
		Some(error.to_string_lossy().into_owned())
	}
	else {
		None
	}
}
