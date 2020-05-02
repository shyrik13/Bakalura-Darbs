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
	"math/rand"
	"runtime"
	"time"
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
	myObject := object.New(vertices, textures, normals)
	myObject.X = -15.0
	myObject.Y = 0.0
	myObject.Z = -15.0

	fmt.Printf("%+v\n", myObject)

	var objects []*object.Object
	objects = append(objects, myObject)

	projection := mgl32.Perspective(mgl32.DegToRad(90.0), 4/3, 0.1, 100.0)
	projectionID := gl.GetUniformLocation(program, gl.Str("perspective\x00"))

	view := mgl32.LookAtV(mgl32.Vec3{1, 0, 1}, mgl32.Vec3{0, 0, 0}, mgl32.Vec3{0, 1, 0})
	viewID := gl.GetUniformLocation(program, gl.Str("view\x00"))

	modelID := gl.GetUniformLocation(program, gl.Str("model\x00"))

	uLight := mgl32.Vec3{0.0, 0.4, 5.7}
	uLightID := gl.GetUniformLocation(program, gl.Str("u_light\x00"))

	textureDiffID := gl.GetUniformLocation(program, gl.Str("diffuse_tex\x00"))
	textureNormID := gl.GetUniformLocation(program, gl.Str("normal_tex\x00"))

	// VERTEX DATA CONFIGURE

	//myObject.InitGlObject(myObject)

	defer gl.DeleteTextures(1, &textureDiff)
	defer gl.DeleteTextures(1, &textureNorm)
	defer gl.DeleteProgram(program)

	var t = 0.0
	var c = math.Cos(t)
	var s = math.Sin(t)

	var nbFrames = 0
	var lastTime = glfw.GetTime()

	var max = float32(15.0)
	var min = float32(-15.0)

	var max0 = float32(0.0)
	var min0 = float32(-30.0)

	polygon := 12
	object_count := 1

	timeNow := time.Now()
	fmt.Printf("%s polygons : %d objects : %d \n", timeNow.Format("2006-01-02 15:04:05"), polygon, object_count)
	setupScene()
	for !window.ShouldClose() {

		if object_count >= 500 {
			window.SetShouldClose(true)
		}

		var currentTime = glfw.GetTime()
		nbFrames++
		if currentTime-lastTime >= 1.0 {
			//fmt.Printf("%f ms/frame \n", 1000.0/(float32(nbFrames)))
			nbFrames = 0
			lastTime += 1.0

			polygon += 12
			object_count++
			timeNow := time.Now()
			fmt.Printf("%s polygons : %d objects : %d \n", timeNow.Format("2006-01-02 15:04:05"), polygon, object_count)

			obj := object.New(vertices, textures, normals)
			obj.X = min0 + rand.Float32()*(max0-min0)
			obj.Y = min + rand.Float32()*(max-min)
			obj.Z = -25.0

			objects = append(objects, obj)
			//obj.InitGlObject(obj)

		}

		gl.Clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
		gl.ClearColor(0.0, 0.0, 0.4, 0.0)

		for _, obj := range objects {

			gl.UseProgram(program)

			gl.GenVertexArrays(1, &obj.VertexArrayID)
			gl.BindVertexArray(obj.VertexArrayID)

			obj.InitGlObjectModel(obj, c, s)

			gl.UniformMatrix4fv(projectionID, 1, false, &projection[0])
			gl.UniformMatrix4fv(viewID, 1, false, &view[0])
			gl.UniformMatrix4fv(modelID, 1, false, &obj.Model[0])
			gl.Uniform3fv(uLightID, 1, &uLight[0])

			gl.ActiveTexture(gl.TEXTURE0)
			gl.BindTexture(gl.TEXTURE_2D, textureDiff)
			gl.Uniform1i(textureDiffID, 0)

			gl.ActiveTexture(gl.TEXTURE1)
			gl.BindTexture(gl.TEXTURE_2D, textureNorm)
			gl.Uniform1i(textureNormID, 1)

			vertAttrib := uint32(gl.GetAttribLocation(program, gl.Str("position\x00")))
			gl.GenBuffers(1, &obj.Vertexbuffer)
			gl.BindBuffer(gl.ARRAY_BUFFER, obj.Vertexbuffer)
			gl.BufferData(gl.ARRAY_BUFFER, len(obj.VertexArray)*4, gl.Ptr(obj.VertexArray), gl.STATIC_DRAW)
			gl.EnableVertexAttribArray(vertAttrib)
			gl.VertexAttribPointer(vertAttrib, 3, gl.FLOAT, false, 0, gl.PtrOffset(0))

			txtAttrib := uint32(gl.GetAttribLocation(program, gl.Str("tex_coords\x00")))
			gl.GenBuffers(1, &obj.Uvbuffer)
			gl.BindBuffer(gl.ARRAY_BUFFER, obj.Uvbuffer)
			gl.BufferData(gl.ARRAY_BUFFER, len(obj.TextureArray)*4, gl.Ptr(obj.TextureArray), gl.STATIC_DRAW)
			gl.EnableVertexAttribArray(txtAttrib)
			gl.VertexAttribPointer(txtAttrib, 2, gl.FLOAT, false, 0, gl.PtrOffset(0))

			normAttrib := uint32(gl.GetAttribLocation(program, gl.Str("normal\x00")))
			gl.GenBuffers(1, &obj.Unbuffer)
			gl.BindBuffer(gl.ARRAY_BUFFER, obj.Unbuffer)
			gl.BufferData(gl.ARRAY_BUFFER, len(obj.NormalArray)*4, gl.Ptr(obj.NormalArray), gl.STATIC_DRAW)
			gl.EnableVertexAttribArray(normAttrib)
			gl.VertexAttribPointer(normAttrib, 3, gl.FLOAT, false, 0, gl.PtrOffset(0))

			fmt.Println(len(obj.VertexArray))
			gl.DrawArrays(gl.TRIANGLES, 0, int32(len(obj.VertexArray)))

			gl.DisableVertexAttribArray(vertAttrib)
			gl.DisableVertexAttribArray(txtAttrib)
			gl.DisableVertexAttribArray(normAttrib)

			gl.DeleteBuffers(1, &obj.Vertexbuffer)
			gl.DeleteBuffers(1, &obj.Uvbuffer)
			gl.DeleteBuffers(1, &obj.Unbuffer)
			gl.DeleteVertexArrays(1, &obj.VertexArrayID)

		}

		window.SwapBuffers()
		glfw.PollEvents()

		t += 0.001
		if t > 360 {
			t = 0
		}
		c = math.Cos(t)
		s = math.Sin(t)

	}

}

func setupScene() {
	gl.Enable(gl.DEPTH_TEST) // Depth Testing
	gl.DepthFunc(gl.LEQUAL)
	gl.Disable(gl.CULL_FACE)
}
