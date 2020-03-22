module read

import os

const (
	BASE = os.dir( os.realpath( os.executable() ) )
	SHADER = './res/shader/'
	FILES  = './res/files/'
)

pub fn file_in_string(file_name string, i int) string {

	s := if (i == 0) {(SHADER + file_name)} 
		 else if (i == 1) {(FILES + file_name)}
		 else {file_name}
		 
	contents := os.read_file(s.trim_space()) or {
		println('failed to open $s')
		return ''
	}
	return contents
}

pub fn read_into_object(contents string) ([]f32, []f32, []f32) {
	
	mut v_array := [[0.0, 0.0, 0.0]]
	mut t_array := [[0.0, 0.0]]
	mut n_array := [[0.0, 0.0, 0.0]]
	
	mut vertices := []f32
	mut textures := []f32
	mut normals := []f32
	
	for line in contents.to_lower().split('\n') {
	
		split := line.split(' ')
		
		if (split.len == 0) {
			break
		}
		
		if (split[0] == 'v') {
			arr := [split[1].f32(), split[2].f32(), split[3].f32()]
			v_array << arr
		} else if (split[0] == 'vt') {
			arr := [split[1].f32(), split[2].f32()]
			t_array << arr
		} else if (split[0] == 'vn') {
			arr := [split[1].f32(), split[2].f32(), split[3].f32()]
			n_array << arr
		} else if (split[0] == 'f') {
			for i := 1; i < split.len; i++ {
				n_split := split[i].split('/')
				vertices << v_array[n_split[0].int()]
				textures << t_array[n_split[1].int()]
				normals << n_array[n_split[2].int()]
			}
		}
	}
	
	return vertices, textures, normals
}
