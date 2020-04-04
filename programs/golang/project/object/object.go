package object

import (
	_ "fmt"
	"github.com/go-gl/gl/v2.1/gl"
	"github.com/go-gl/mathgl/mgl32"
	"math"
)

type Object struct {
	NormalArray   []float32 // light vector coords (1 per side)
	VertexArray   []float32 // vertices coords
	TextureArray  []float32 // texture coords
	Model         mgl32.Mat4
	VertexArrayID uint32
	Vertexbuffer  uint32
	Uvbuffer      uint32
	Unbuffer      uint32
	X             float32
	Y             float32
	Z             float32
}

func New(VertexArray, TextureArray, NormalArray []float32) *Object {
	object := new(Object)
	object.VertexArray = VertexArray
	object.TextureArray = TextureArray
	object.NormalArray = NormalArray
	return object
}

func (o Object) InitGlObject(object *Object) {
	// VERTEX DATA CONFIGURE
	gl.GenVertexArrays(1, &object.VertexArrayID)
	gl.BindVertexArray(object.VertexArrayID)

	gl.GenBuffers(1, &object.Vertexbuffer)
	gl.BindBuffer(gl.ARRAY_BUFFER, object.Vertexbuffer)
	gl.BufferData(gl.ARRAY_BUFFER, len(object.VertexArray)*4, gl.Ptr(object.VertexArray), gl.STATIC_DRAW)

	gl.GenBuffers(1, &object.Uvbuffer)
	gl.BindBuffer(gl.ARRAY_BUFFER, object.Uvbuffer)
	gl.BufferData(gl.ARRAY_BUFFER, len(object.TextureArray)*4, gl.Ptr(object.TextureArray), gl.STATIC_DRAW)

	gl.GenBuffers(1, &object.Unbuffer)
	gl.BindBuffer(gl.ARRAY_BUFFER, object.Unbuffer)
	gl.BufferData(gl.ARRAY_BUFFER, len(object.NormalArray)*4, gl.Ptr(object.NormalArray), gl.STATIC_DRAW)
}

func (o Object) InitGlObjectModel(object *Object, c, s float64) {
	object.Model = mgl32.Mat4{
		float32(math.Pow(c, 2)), float32(-c * s), float32(s), 0.0,
		float32(c * (math.Pow(s, 2) + s)), float32(math.Pow(c, 2) - math.Pow(s, 3)), float32(-c * s), 0.0,
		float32(s * (s - math.Pow(c, 2))), float32(c * (math.Pow(s, 2) + s)), float32(math.Pow(c, 2)), 0.0,
		object.X, object.Y, object.Z, 1.0,
	}
}
