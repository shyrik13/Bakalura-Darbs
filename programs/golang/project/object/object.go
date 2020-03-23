package object

import (
	_ "fmt"
)

type Object struct {
	NormalArray  []float32 // light vector coords (1 per side)
	VertexArray  []float32 // vertices coords
	TextureArray []float32 // texture coords
	TextureDiff  uint32    // texture diffuse
	TextureNorm  uint32    // texture normal map
	Program      uint32    // shader program
}

func New(VertexArray, TextureArray, NormalArray []float32, textureDiff, textureNorm, program uint32) *Object {
	object := new(Object)
	object.VertexArray = VertexArray
	object.TextureArray = TextureArray
	object.NormalArray = NormalArray
	object.TextureDiff = textureDiff
	object.TextureNorm = textureNorm
	object.Program = program
	return object
}
