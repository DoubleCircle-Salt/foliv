package foliv

import (
	"os"
	"testing"

	"github.com/golang/protobuf/proto"
)

func TestMarshalAndUnmarshal(t *testing.T) {
	testMarshalAndUnmarshal(t)
}

func TestMarshalToFile(t *testing.T) {
	//testMarshalToFile(t)
}

func TestUnmarshalFromFile(t *testing.T) {
	//testUnmarshalFromFile(t)
}

func testMarshalAndUnmarshal(t *testing.T) {
	foliv := &Foliv{
		UserHash:    "yyytest123",
		Command:     Command_Connect,
		AddressType: AddressType_IPv4,
		Address:     []byte("12345"),
		SourceName:  "Hongkong",
	}
	data, err := proto.Marshal(foliv)
	if err != nil {
		t.Error(err)
	}

	println("length:", len(data))

	newFoliv := &Foliv{}
	if err := proto.Unmarshal(data, newFoliv); err != nil {
		t.Error(err)
	}

	println("println:", newFoliv.String())
}

func testMarshalToFile(t *testing.T) {
	foliv := &Foliv{
		UserHash:    "yyytest123",
		Command:     Command_Connect,
		AddressType: AddressType_IPv4,
		Address:     []byte("12345"),
		SourceName:  "Hongkong",
	}
	file, err := os.Create(".test")
	if err != nil {
		t.Error(err)
	}
	data, err := proto.Marshal(foliv)
	if err != nil {
		t.Error(err)
	}
	if _, err := file.Write(data); err != nil {
		t.Error(err)
	}
}

func testUnmarshalFromFile(t *testing.T) {
	file, err := os.Open(".test")
	if err != nil {
		t.Error(err)
	}
	payload := [1024]byte{}
	n, err := file.Read(payload[:])
	if err != nil {
		t.Error(err)
	}
	newFoliv := &Foliv{}
	if err := proto.Unmarshal(payload[:n], newFoliv); err != nil {
		t.Error(err)
	}
	println("println:", newFoliv.String())
}

