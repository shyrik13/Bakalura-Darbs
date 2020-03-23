package main

import "C"
import (
	"./common"
	"./object"
	"fmt"
	_ "fmt"
	"github.com/go-gl/gl/v2.1/gl"
	"github.com/go-gl/glfw/v3.3/glfw"
	"github.com/go-gl/mathgl/mgl32"
	_ "go/build"
	_ "image/png"
	"log"
	"math"
	"runtime"
)

const width, height = 1200, 600

func init() {
	// GLFW event handling must run on the main OS thread
	runtime.LockOSThread()
}

func main() {

	if err := glfw.Init(); err != nil {
		log.Fatalln("failed to initialize glfw:", err)
	}
	defer glfw.Terminate()

	glfw.WindowHint(glfw.Resizable, glfw.False)
	glfw.WindowHint(glfw.ContextVersionMajor, 4)
	glfw.WindowHint(glfw.ContextVersionMinor, 1)
	glfw.WindowHint(glfw.OpenGLProfile, glfw.OpenGLCoreProfile)
	glfw.WindowHint(glfw.OpenGLForwardCompatible, glfw.True)
	window, err := glfw.CreateWindow(width, height, "Cube", nil, nil)
	if err != nil {
		panic(err)
	}
	window.MakeContextCurrent()

	if err := gl.Init(); err != nil {
		panic(err)
	}

	version := gl.GoStr(gl.GetString(gl.VERSION))
	fmt.Println("OpenGL version", version)

	vertices, textures, normals := common.ReadIntoArrays(common.FileInString("cube.obj", 1))

	textureDiff, err := common.NewTexture("tuto-14-diffuse.png")
	if err != nil {
		panic(err)
	}
	textureNorm, err := common.NewTexture("tuto-14-normal.png")
	if err != nil {
		panic(err)
	}

	program, err := common.NewProgram(common.FileInString("vertex_shader.txt", 0), common.FileInString("fragment_shader.txt", 0))
	myObject := object.New(vertices, textures, normals, textureDiff, textureNorm, program)

	projection := mgl32.Perspective(mgl32.DegToRad(45.0), 4/3, 0.1, 100.0)
	projectionID := gl.GetUniformLocation(program, gl.Str("perspective\x00"))

	view := mgl32.LookAtV(mgl32.Vec3{4, 3, 3}, mgl32.Vec3{0, 0, 0}, mgl32.Vec3{0, 1, 0})
	viewID := gl.GetUniformLocation(program, gl.Str("view\x00"))

	modelID := gl.GetUniformLocation(program, gl.Str("model\x00"))

	uLight := mgl32.Vec3{0.0, 0.4, 5.7}
	uLightID := gl.GetUniformLocation(program, gl.Str("u_light\x00"))

	textureDiffID := gl.GetUniformLocation(program, gl.Str("diffuse_tex\x00"))
	textureNormID := gl.GetUniformLocation(program, gl.Str("normal_tex\x00"))

	// VERTEX DATA CONFIGURE
	var vao uint32
	gl.GenVertexArrays(1, &vao)
	gl.BindVertexArray(vao)

	var vboV uint32
	gl.GenBuffers(1, &vboV)

	var vboT uint32
	gl.GenBuffers(1, &vboT)

	var vboN uint32
	gl.GenBuffers(1, &vboN)

	defer gl.DeleteTextures(1, &myObject.TextureDiff)
	defer gl.DeleteTextures(1, &myObject.TextureNorm)
	defer gl.DeleteBuffers(1, &vboV)
	defer gl.DeleteBuffers(1, &vboT)
	defer gl.DeleteBuffers(1, &vboN)
	defer gl.DeleteBuffers(1, &vao)
	defer gl.DeleteProgram(myObject.Program)

	var t = 0.0
	var c = math.Cos(t)
	var s = math.Sin(t)

	setupScene()
	for !window.ShouldClose() {

		gl.Clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
		gl.ClearColor(0.0, 0.0, 0.4, 0.0)
		gl.UseProgram(myObject.Program)

		t += 0.001
		if t > 360 {
			t = 0
		}
		c = math.Cos(t)
		s = math.Sin(t)

		model := mgl32.Mat4{
			float32(math.Pow(c, 2)), float32(-c * s), float32(s), 0.0,
			float32(c * (math.Pow(s, 2) + s)), float32(math.Pow(c, 2) - math.Pow(s, 3)), float32(-c * s), 0.0,
			float32(s * (s - math.Pow(c, 2))), float32(c * (math.Pow(s, 2) + s)), float32(math.Pow(c, 2)), 0.0,
			-3.0, 0.0, 0.0, 1.0,
		}

		gl.UniformMatrix4fv(projectionID, 1, false, &projection[0])
		gl.UniformMatrix4fv(viewID, 1, false, &view[0])
		gl.UniformMatrix4fv(modelID, 1, false, &model[0])
		gl.Uniform3fv(uLightID, 1, &uLight[0])

		gl.ActiveTexture(gl.TEXTURE0)
		gl.BindTexture(gl.TEXTURE_2D, myObject.TextureDiff)
		gl.Uniform1i(textureDiffID, 0)

		gl.ActiveTexture(gl.TEXTURE1)
		gl.BindTexture(gl.TEXTURE_2D, myObject.TextureNorm)
		gl.Uniform1i(textureNormID, 1)

		vertAttrib := uint32(gl.GetAttribLocation(program, gl.Str("position\x00")))
		gl.BindBuffer(gl.ARRAY_BUFFER, vboV)
		gl.BufferData(gl.ARRAY_BUFFER, len(myObject.VertexArray)*4, gl.Ptr(myObject.VertexArray), gl.STATIC_DRAW)
		gl.EnableVertexAttribArray(vertAttrib)
		gl.VertexAttribPointer(vertAttrib, 3, gl.FLOAT, false, 0, gl.PtrOffset(0))

		txtAttrib := uint32(gl.GetAttribLocation(program, gl.Str("tex_coords\x00")))
		gl.BindBuffer(gl.ARRAY_BUFFER, vboT)
		gl.BufferData(gl.ARRAY_BUFFER, len(myObject.TextureArray)*4, gl.Ptr(myObject.TextureArray), gl.STATIC_DRAW)
		gl.EnableVertexAttribArray(txtAttrib)
		gl.VertexAttribPointer(txtAttrib, 2, gl.FLOAT, false, 0, gl.PtrOffset(0))

		normAttrib := uint32(gl.GetAttribLocation(program, gl.Str("normal\x00")))
		gl.BindBuffer(gl.ARRAY_BUFFER, vboN)
		gl.BufferData(gl.ARRAY_BUFFER, len(myObject.NormalArray)*4, gl.Ptr(myObject.NormalArray), gl.STATIC_DRAW)
		gl.EnableVertexAttribArray(normAttrib)
		gl.VertexAttribPointer(normAttrib, 3, gl.FLOAT, false, 0, gl.PtrOffset(0))

		gl.DrawArrays(gl.TRIANGLES, 0, int32(len(myObject.VertexArray)))

		window.SwapBuffers()
		glfw.PollEvents()

		gl.DisableVertexAttribArray(vertAttrib)
		gl.DisableVertexAttribArray(txtAttrib)
		gl.DisableVertexAttribArray(normAttrib)
	}
}

func setupScene() {
	gl.Enable(gl.DEPTH_TEST) // Depth Testing
	gl.DepthFunc(gl.LEQUAL)
	gl.Disable(gl.CULL_FACE)
}
