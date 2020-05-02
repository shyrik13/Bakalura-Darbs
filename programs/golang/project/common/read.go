package common

import (
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

const (
	FILES  = "./res/files/"
	SHADER = "./res/shader/"
)

func FileInString(fileName string, i int) string {
	path := FILES + fileName
	if i == 0 {
		path = SHADER + fileName
	}
	content, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	text := string(content)
	return text
}

func ReadIntoArrays(content string) ([]float32, []float32, []float32) {

	var vArray = [][]float32{{0.0, 0.0, 0.0}}
	var tArray = [][]float32{{0.0, 0.0}}
	var nArray = [][]float32{{0.0, 0.0, 0.0}}

	var vertices []float32
	var textures []float32
	var normals []float32

	for _, line := range strings.Split(content, "\n") {

		split := strings.Split(line, " ")

		if len(split) == 0 {
			break
		}

		if split[0] == "v" {
			f1, _ := strconv.ParseFloat(split[1], 32)
			f2, _ := strconv.ParseFloat(split[2], 32)
			f3, _ := strconv.ParseFloat(split[3], 32)
			arr := []float32{float32(f1), float32(f2), float32(f3)}
			vArray = append(vArray, arr)
		} else if split[0] == "vt" {
			f1, _ := strconv.ParseFloat(split[1], 32)
			f2, _ := strconv.ParseFloat(split[2], 32)
			arr := []float32{float32(f1), float32(f2)}
			tArray = append(tArray, arr)
		} else if split[0] == "vn" {
			f1, _ := strconv.ParseFloat(split[1], 32)
			f2, _ := strconv.ParseFloat(split[2], 32)
			f3, _ := strconv.ParseFloat(split[3], 32)
			arr := []float32{float32(f1), float32(f2), float32(f3)}
			nArray = append(nArray, arr)
		} else if split[0] == "f" {
			for i := 1; i < len(split); i++ {
				nSplit := strings.Split(split[i], "/")
				f1, _ := strconv.Atoi(nSplit[0])
				f2, _ := strconv.Atoi(nSplit[1])
				f3, _ := strconv.Atoi(nSplit[2])
				vertices = append(vertices, vArray[f1][0], vArray[f1][1], vArray[f1][2])
				textures = append(textures, tArray[f2][0], tArray[f2][1])
				normals = append(normals, nArray[f3][0], nArray[f3][1], nArray[f3][2])
			}
		}

	}

	return vertices, textures, normals
}
