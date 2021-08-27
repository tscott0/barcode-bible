package main

import (
	"fmt"
	"github.com/boombuler/barcode"
	"github.com/boombuler/barcode/code128"
	"github.com/boombuler/barcode/ean"
	"github.com/signintech/gopdf"
	"image/jpeg"
	"log"
	"os"
	"path/filepath"
)

const (
	imageDir      = "images"
	barcodeWidth  = 500
	barcodeHeight = 200
	fontName      = "go"
	fontFilePath  = "Go-Regular.ttf"
)

var xPos float64 = 20
var yPos float64 = 20

const (
	EAN8 = iota
	GS1_128
)

func main() {
	err := os.MkdirAll(imageDir, os.ModePerm)
	if err != nil {
		log.Fatal("Failed to create image dir")
	}

	// PDF
	pdf := gopdf.GoPdf{}
	pdf.Start(gopdf.Config{PageSize: *gopdf.PageSizeA4})
	pdf.AddPage()

	pdf.SetX(xPos)
	pdf.SetY(yPos)

	err = pdf.AddTTFFont(fontName, fontFilePath)
	if err != nil {
		log.Fatalf("Failed to add the font: %s", err)
	}

	err = pdf.SetFont(fontName, "", 14)
	if err != nil {
		log.Fatalf("Failed to set the font: %s", err)
	}

	codes := map[string]int{
		"00012345":      EAN8,
		"1212526228612": GS1_128,
	}

	for c, codeType := range codes {
		err = writeCode(c, codeType, &pdf)
		if err != nil {
			log.Fatalf("Failed to write code %s: %s", c, err)
		}
	}

	err = pdf.WritePdf("test.pdf")
	if err != nil {
		log.Fatal("Failed to write PDF")
	}
}

func writeCode(code string, codeType int, pdf *gopdf.GoPdf) error {
	var b barcode.Barcode
	var err error

	switch codeType {
	case EAN8:
		{
			b, err = ean.Encode(code)
			if err != nil {
				log.Fatal("Failed to encode EAN")
			}
		}
	case GS1_128:
		{
			b, err = code128.Encode(code)
			if err != nil {
				log.Fatal("Failed to encode GS1_128")
			}
		}
	}

	scaledBarcode, err := barcode.Scale(b, barcodeWidth, barcodeHeight)

	// create the output file
	imagePath := filepath.Clean(fmt.Sprintf("%s/%s.jpg", imageDir, code))
	file, _ := os.Create(imagePath)
	defer file.Close()

	// encode the barcode as jpeg
	err = jpeg.Encode(file, scaledBarcode, &jpeg.Options{Quality: 100})
	if err != nil {
		log.Fatalf("Failed to write barcode to file: %s", err)
	}

	err = pdf.Image(imagePath, xPos, yPos, nil)
	if err != nil {
		log.Fatalf("Failed to write image to PDF: %s", err)
	}

	yPos = yPos + barcodeHeight - 50
	//pdf.SetY(yPos)
	//
	//pdf.Cell(nil, code) //print text
	//
	//yPos = yPos + 20

	return err
}
