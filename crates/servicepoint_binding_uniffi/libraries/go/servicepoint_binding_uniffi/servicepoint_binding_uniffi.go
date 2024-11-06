
package servicepoint_binding_uniffi

// #include <servicepoint_binding_uniffi.h>
import "C"

import (
	"bytes"
	"fmt"
	"io"
	"unsafe"
	"encoding/binary"
	"math"
	"runtime"
	"sync/atomic"
)



type RustBuffer = C.RustBuffer

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() int
	Capacity() int
}

func RustBufferFromExternal(b RustBufferI) RustBuffer {
	return RustBuffer {
		capacity: C.int(b.Capacity()),
		len: C.int(b.Len()),
		data: (*C.uchar)(b.Data()),
	}
}

func (cb RustBuffer) Capacity() int {
	return int(cb.capacity)
}

func (cb RustBuffer) Len() int {
	return int(cb.len)
}

func (cb RustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.data)
}

func (cb RustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.data), C.int(cb.len))
	return bytes.NewReader(b)
}

func (cb RustBuffer) Free() {
	rustCall(func( status *C.RustCallStatus) bool {
		C.ffi_servicepoint_binding_uniffi_rustbuffer_free(cb, status)
		return false
	})
}

func (cb RustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.data), C.int(cb.len))
}


func stringToRustBuffer(str string) RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) RustBuffer {
	if len(b) == 0 {
		return RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes {
		len: C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}
	
	return rustCall(func( status *C.RustCallStatus) RustBuffer {
		return C.ffi_servicepoint_binding_uniffi_rustbuffer_from_bytes(foreign, status)
	})
}



type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType) RustBuffer
}

type FfiConverter[GoType any, FfiType any] interface {
	Lift(value FfiType) GoType
	Lower(value GoType) FfiType
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

type FfiRustBufConverter[GoType any, FfiType any] interface {
	FfiConverter[GoType, FfiType]
	BufReader[GoType]
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}



func rustCallWithError[U any](converter BufLifter[error], callback func(*C.RustCallStatus) U) (U, error) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)

	return returnValue, err
}

func checkCallStatus(converter BufLifter[error], status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		return converter.Lift(status.errorBuf)
	case 2:
		// when the rust code sees a panic, it tries to construct a rustbuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(status.errorBuf)))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct a rustbuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(status.errorBuf)))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError(nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}


func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}


func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {
        
        uniffiCheckChecksums()
}


func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 24
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_servicepoint_binding_uniffi_uniffi_contract_version(uniffiStatus)
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: UniFFI contract version mismatch")
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_copy_raw(uniffiStatus)
	})
	if checksum != 12617 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_copy_raw: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_equals(uniffiStatus)
	})
	if checksum != 1191 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_equals: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_fill(uniffiStatus)
	})
	if checksum != 12255 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_fill: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_get(uniffiStatus)
	})
	if checksum != 43835 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_get: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_len(uniffiStatus)
	})
	if checksum != 22196 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_len: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_set(uniffiStatus)
	})
	if checksum != 16307 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_set: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_copy_raw(uniffiStatus)
	})
	if checksum != 3467 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_copy_raw: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_equals(uniffiStatus)
	})
	if checksum != 420 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_equals: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_fill(uniffiStatus)
	})
	if checksum != 43887 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_fill: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_get(uniffiStatus)
	})
	if checksum != 61136 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_get: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_height(uniffiStatus)
	})
	if checksum != 44991 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_height: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_set(uniffiStatus)
	})
	if checksum != 25290 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_set: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_width(uniffiStatus)
	})
	if checksum != 30837 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_width: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_copy_raw(uniffiStatus)
	})
	if checksum != 28155 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_copy_raw: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_equals(uniffiStatus)
	})
	if checksum != 13314 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_equals: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_fill(uniffiStatus)
	})
	if checksum != 63376 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_fill: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_get(uniffiStatus)
	})
	if checksum != 28736 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_get: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_height(uniffiStatus)
	})
	if checksum != 39528 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_height: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_set(uniffiStatus)
	})
	if checksum != 6330 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_set: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_width(uniffiStatus)
	})
	if checksum != 26384 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_brightnessgrid_width: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_command_equals(uniffiStatus)
	})
	if checksum != 20763 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_command_equals: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_connection_send(uniffiStatus)
	})
	if checksum != 23796 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_connection_send: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_copy_raw(uniffiStatus)
	})
	if checksum != 50937 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_copy_raw: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_equals(uniffiStatus)
	})
	if checksum != 21544 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_equals: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_fill(uniffiStatus)
	})
	if checksum != 46422 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_fill: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_get(uniffiStatus)
	})
	if checksum != 1945 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_get: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_height(uniffiStatus)
	})
	if checksum != 45951 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_height: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_set(uniffiStatus)
	})
	if checksum != 8371 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_set: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_width(uniffiStatus)
	})
	if checksum != 36872 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_cp437grid_width: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_clone(uniffiStatus)
	})
	if checksum != 123 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_clone: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_load(uniffiStatus)
	})
	if checksum != 48913 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_load: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_new(uniffiStatus)
	})
	if checksum != 11865 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_new: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_clone(uniffiStatus)
	})
	if checksum != 57298 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_clone: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_load(uniffiStatus)
	})
	if checksum != 24109 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_load: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new(uniffiStatus)
	})
	if checksum != 49832 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new_max_sized(uniffiStatus)
	})
	if checksum != 63762 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new_max_sized: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_clone(uniffiStatus)
	})
	if checksum != 33422 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_clone: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_load(uniffiStatus)
	})
	if checksum != 24788 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_load: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_new(uniffiStatus)
	})
	if checksum != 4979 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_brightnessgrid_new: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear(uniffiStatus)
	})
	if checksum != 18079 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_and(uniffiStatus)
	})
	if checksum != 18147 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_and: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_or(uniffiStatus)
	})
	if checksum != 44912 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_or: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_win(uniffiStatus)
	})
	if checksum != 24563 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_win: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_xor(uniffiStatus)
	})
	if checksum != 54278 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_xor: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_brightness(uniffiStatus)
	})
	if checksum != 11291 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_brightness: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_char_brightness(uniffiStatus)
	})
	if checksum != 29467 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_char_brightness: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear(uniffiStatus)
	})
	if checksum != 11035 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clone(uniffiStatus)
	})
	if checksum != 42249 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clone: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_cp437_data(uniffiStatus)
	})
	if checksum != 33157 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_cp437_data: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_fade_out(uniffiStatus)
	})
	if checksum != 49231 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_fade_out: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_hard_reset(uniffiStatus)
	})
	if checksum != 62130 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_hard_reset: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new(uniffiStatus)
	})
	if checksum != 30445 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new_fake(uniffiStatus)
	})
	if checksum != 54331 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new_fake: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_clone(uniffiStatus)
	})
	if checksum != 28173 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_clone: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_load(uniffiStatus)
	})
	if checksum != 62136 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_load: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_new(uniffiStatus)
	})
	if checksum != 17350 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_cp437grid_new: UniFFI API checksum mismatch")
	}
	}
}




type FfiConverterUint8 struct{}

var FfiConverterUint8INSTANCE = FfiConverterUint8{}

func (FfiConverterUint8) Lower(value uint8) C.uint8_t {
	return C.uint8_t(value)
}

func (FfiConverterUint8) Write(writer io.Writer, value uint8) {
	writeUint8(writer, value)
}

func (FfiConverterUint8) Lift(value C.uint8_t) uint8 {
	return uint8(value)
}

func (FfiConverterUint8) Read(reader io.Reader) uint8 {
	return readUint8(reader)
}

type FfiDestroyerUint8 struct {}

func (FfiDestroyerUint8) Destroy(_ uint8) {}


type FfiConverterUint64 struct{}

var FfiConverterUint64INSTANCE = FfiConverterUint64{}

func (FfiConverterUint64) Lower(value uint64) C.uint64_t {
	return C.uint64_t(value)
}

func (FfiConverterUint64) Write(writer io.Writer, value uint64) {
	writeUint64(writer, value)
}

func (FfiConverterUint64) Lift(value C.uint64_t) uint64 {
	return uint64(value)
}

func (FfiConverterUint64) Read(reader io.Reader) uint64 {
	return readUint64(reader)
}

type FfiDestroyerUint64 struct {}

func (FfiDestroyerUint64) Destroy(_ uint64) {}


type FfiConverterBool struct{}

var FfiConverterBoolINSTANCE = FfiConverterBool{}

func (FfiConverterBool) Lower(value bool) C.int8_t {
	if value {
		return C.int8_t(1)
	}
	return C.int8_t(0)
}

func (FfiConverterBool) Write(writer io.Writer, value bool) {
	if value {
		writeInt8(writer, 1)
	} else {
		writeInt8(writer, 0)
	}
}

func (FfiConverterBool) Lift(value C.int8_t) bool {
	return value != 0
}

func (FfiConverterBool) Read(reader io.Reader) bool {
	return readInt8(reader) != 0
}

type FfiDestroyerBool struct {}

func (FfiDestroyerBool) Destroy(_ bool) {}


type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) RustBuffer {
	return stringToRustBuffer(value)
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct {}

func (FfiDestroyerString) Destroy(_ string) {}


type FfiConverterBytes struct{}

var FfiConverterBytesINSTANCE = FfiConverterBytes{}

func (c FfiConverterBytes) Lower(value []byte) RustBuffer {
	return LowerIntoRustBuffer[[]byte](c, value)
}

func (c FfiConverterBytes) Write(writer io.Writer, value []byte) {
	if len(value) > math.MaxInt32 {
		panic("[]byte is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := writer.Write(value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing []byte, expected %d, written %d", len(value), write_length))
	}
}

func (c FfiConverterBytes) Lift(rb RustBufferI) []byte {
	return LiftFromRustBuffer[[]byte](c, rb)
}

func (c FfiConverterBytes) Read(reader io.Reader) []byte {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading []byte, expected %d, read %d", length, read_length))
	}
	return buffer
}

type FfiDestroyerBytes struct {}

func (FfiDestroyerBytes) Destroy(_ []byte) {}




// Below is an implementation of synchronization requirements outlined in the link.
// https://github.com/mozilla/uniffi-rs/blob/0dc031132d9493ca812c3af6e7dd60ad2ea95bf0/uniffi_bindgen/src/bindings/kotlin/templates/ObjectRuntime.kt#L31

type FfiObject struct {
	pointer unsafe.Pointer
	callCounter atomic.Int64
	freeFunction func(unsafe.Pointer, *C.RustCallStatus)
	destroyed atomic.Bool
}

func newFfiObject(pointer unsafe.Pointer, freeFunction func(unsafe.Pointer, *C.RustCallStatus)) FfiObject {
	return FfiObject {
		pointer: pointer,
		freeFunction: freeFunction,
	}
}

func (ffiObject *FfiObject)incrementPointer(debugName string) unsafe.Pointer {
	for {
		counter := ffiObject.callCounter.Load()
		if counter <= -1 {
			panic(fmt.Errorf("%v object has already been destroyed", debugName))
		}
		if counter == math.MaxInt64 {
			panic(fmt.Errorf("%v object call counter would overflow", debugName))
		}
		if ffiObject.callCounter.CompareAndSwap(counter, counter + 1) {
			break
		}
	}

	return ffiObject.pointer
}

func (ffiObject *FfiObject)decrementPointer() {
	if ffiObject.callCounter.Add(-1) == -1 {
		ffiObject.freeRustArcPtr()
	}
}

func (ffiObject *FfiObject)destroy() {
	if ffiObject.destroyed.CompareAndSwap(false, true) {
		if ffiObject.callCounter.Add(-1) == -1 {
			ffiObject.freeRustArcPtr()
		}
	}
}

func (ffiObject *FfiObject)freeRustArcPtr() {
	rustCall(func(status *C.RustCallStatus) int32 {
		ffiObject.freeFunction(ffiObject.pointer, status)
		return 0
	})
}
type BitVec struct {
	ffiObject FfiObject
}
func NewBitVec(size uint64) *BitVec {
	return FfiConverterBitVecINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_new(FfiConverterUint64INSTANCE.Lower(size), _uniffiStatus)
	}))
}


func BitVecClone(other *BitVec) *BitVec {
	return FfiConverterBitVecINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_clone(FfiConverterBitVecINSTANCE.Lower(other), _uniffiStatus)
	}))
}

func BitVecLoad(data []byte) *BitVec {
	return FfiConverterBitVecINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_load(FfiConverterBytesINSTANCE.Lower(data), _uniffiStatus)
	}))
}



func (_self *BitVec)CopyRaw() []byte {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBytesINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_copy_raw(
		_pointer, _uniffiStatus)
	}))
}


func (_self *BitVec)Equals(other *BitVec) bool {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_equals(
		_pointer,FfiConverterBitVecINSTANCE.Lower(other), _uniffiStatus)
	}))
}


func (_self *BitVec)Fill(value bool)  {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_fill(
		_pointer,FfiConverterBoolINSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *BitVec)Get(index uint64) bool {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_get(
		_pointer,FfiConverterUint64INSTANCE.Lower(index), _uniffiStatus)
	}))
}


func (_self *BitVec)Len() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_len(
		_pointer, _uniffiStatus)
	}))
}


func (_self *BitVec)Set(index uint64, value bool)  {
	_pointer := _self.ffiObject.incrementPointer("*BitVec")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_bitvec_set(
		_pointer,FfiConverterUint64INSTANCE.Lower(index), FfiConverterBoolINSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}



func (object *BitVec)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterBitVec struct {}

var FfiConverterBitVecINSTANCE = FfiConverterBitVec{}

func (c FfiConverterBitVec) Lift(pointer unsafe.Pointer) *BitVec {
	result := &BitVec {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_bitvec(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*BitVec).Destroy)
	return result
}

func (c FfiConverterBitVec) Read(reader io.Reader) *BitVec {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterBitVec) Lower(value *BitVec) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*BitVec")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterBitVec) Write(writer io.Writer, value *BitVec) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerBitVec struct {}

func (_ FfiDestroyerBitVec) Destroy(value *BitVec) {
	value.Destroy()
}


type Bitmap struct {
	ffiObject FfiObject
}
func NewBitmap(width uint64, height uint64) *Bitmap {
	return FfiConverterBitmapINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), _uniffiStatus)
	}))
}


func BitmapClone(other *Bitmap) *Bitmap {
	return FfiConverterBitmapINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_clone(FfiConverterBitmapINSTANCE.Lower(other), _uniffiStatus)
	}))
}

func BitmapLoad(width uint64, height uint64, data []byte) *Bitmap {
	return FfiConverterBitmapINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_load(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), FfiConverterBytesINSTANCE.Lower(data), _uniffiStatus)
	}))
}

func BitmapNewMaxSized() *Bitmap {
	return FfiConverterBitmapINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new_max_sized( _uniffiStatus)
	}))
}



func (_self *Bitmap)CopyRaw() []byte {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBytesINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_copy_raw(
		_pointer, _uniffiStatus)
	}))
}


func (_self *Bitmap)Equals(other *Bitmap) bool {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_equals(
		_pointer,FfiConverterBitmapINSTANCE.Lower(other), _uniffiStatus)
	}))
}


func (_self *Bitmap)Fill(value bool)  {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_fill(
		_pointer,FfiConverterBoolINSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *Bitmap)Get(x uint64, y uint64) bool {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_get(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), _uniffiStatus)
	}))
}


func (_self *Bitmap)Height() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_height(
		_pointer, _uniffiStatus)
	}))
}


func (_self *Bitmap)Set(x uint64, y uint64, value bool)  {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_set(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), FfiConverterBoolINSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *Bitmap)Width() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*Bitmap")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_bitmap_width(
		_pointer, _uniffiStatus)
	}))
}



func (object *Bitmap)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterBitmap struct {}

var FfiConverterBitmapINSTANCE = FfiConverterBitmap{}

func (c FfiConverterBitmap) Lift(pointer unsafe.Pointer) *Bitmap {
	result := &Bitmap {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_bitmap(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*Bitmap).Destroy)
	return result
}

func (c FfiConverterBitmap) Read(reader io.Reader) *Bitmap {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterBitmap) Lower(value *Bitmap) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Bitmap")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterBitmap) Write(writer io.Writer, value *Bitmap) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerBitmap struct {}

func (_ FfiDestroyerBitmap) Destroy(value *Bitmap) {
	value.Destroy()
}


type BrightnessGrid struct {
	ffiObject FfiObject
}
func NewBrightnessGrid(width uint64, height uint64) *BrightnessGrid {
	return FfiConverterBrightnessGridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_new(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), _uniffiStatus)
	}))
}


func BrightnessGridClone(other *BrightnessGrid) *BrightnessGrid {
	return FfiConverterBrightnessGridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_clone(FfiConverterBrightnessGridINSTANCE.Lower(other), _uniffiStatus)
	}))
}

func BrightnessGridLoad(width uint64, height uint64, data []byte) *BrightnessGrid {
	return FfiConverterBrightnessGridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_brightnessgrid_load(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), FfiConverterBytesINSTANCE.Lower(data), _uniffiStatus)
	}))
}



func (_self *BrightnessGrid)CopyRaw() []byte {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBytesINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_copy_raw(
		_pointer, _uniffiStatus)
	}))
}


func (_self *BrightnessGrid)Equals(other *BrightnessGrid) bool {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_equals(
		_pointer,FfiConverterBrightnessGridINSTANCE.Lower(other), _uniffiStatus)
	}))
}


func (_self *BrightnessGrid)Fill(value uint8)  {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_fill(
		_pointer,FfiConverterUint8INSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *BrightnessGrid)Get(x uint64, y uint64) uint8 {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint8INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_get(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), _uniffiStatus)
	}))
}


func (_self *BrightnessGrid)Height() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_height(
		_pointer, _uniffiStatus)
	}))
}


func (_self *BrightnessGrid)Set(x uint64, y uint64, value uint8)  {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_set(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), FfiConverterUint8INSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *BrightnessGrid)Width() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*BrightnessGrid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_brightnessgrid_width(
		_pointer, _uniffiStatus)
	}))
}



func (object *BrightnessGrid)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterBrightnessGrid struct {}

var FfiConverterBrightnessGridINSTANCE = FfiConverterBrightnessGrid{}

func (c FfiConverterBrightnessGrid) Lift(pointer unsafe.Pointer) *BrightnessGrid {
	result := &BrightnessGrid {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_brightnessgrid(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*BrightnessGrid).Destroy)
	return result
}

func (c FfiConverterBrightnessGrid) Read(reader io.Reader) *BrightnessGrid {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterBrightnessGrid) Lower(value *BrightnessGrid) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*BrightnessGrid")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterBrightnessGrid) Write(writer io.Writer, value *BrightnessGrid) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerBrightnessGrid struct {}

func (_ FfiDestroyerBrightnessGrid) Destroy(value *BrightnessGrid) {
	value.Destroy()
}


type Command struct {
	ffiObject FfiObject
}


func CommandBitmapLinear(offset uint64, bitmap *BitVec, compression CompressionCode) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear(FfiConverterUint64INSTANCE.Lower(offset), FfiConverterBitVecINSTANCE.Lower(bitmap), FfiConverterTypeCompressionCodeINSTANCE.Lower(compression), _uniffiStatus)
	}))
}

func CommandBitmapLinearAnd(offset uint64, bitmap *BitVec, compression CompressionCode) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_and(FfiConverterUint64INSTANCE.Lower(offset), FfiConverterBitVecINSTANCE.Lower(bitmap), FfiConverterTypeCompressionCodeINSTANCE.Lower(compression), _uniffiStatus)
	}))
}

func CommandBitmapLinearOr(offset uint64, bitmap *BitVec, compression CompressionCode) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_or(FfiConverterUint64INSTANCE.Lower(offset), FfiConverterBitVecINSTANCE.Lower(bitmap), FfiConverterTypeCompressionCodeINSTANCE.Lower(compression), _uniffiStatus)
	}))
}

func CommandBitmapLinearWin(offsetX uint64, offsetY uint64, bitmap *Bitmap, compression CompressionCode) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_win(FfiConverterUint64INSTANCE.Lower(offsetX), FfiConverterUint64INSTANCE.Lower(offsetY), FfiConverterBitmapINSTANCE.Lower(bitmap), FfiConverterTypeCompressionCodeINSTANCE.Lower(compression), _uniffiStatus)
	}))
}

func CommandBitmapLinearXor(offset uint64, bitmap *BitVec, compression CompressionCode) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_xor(FfiConverterUint64INSTANCE.Lower(offset), FfiConverterBitVecINSTANCE.Lower(bitmap), FfiConverterTypeCompressionCodeINSTANCE.Lower(compression), _uniffiStatus)
	}))
}

func CommandBrightness(brightness uint8) (*Command, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeServicePointError{},func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_brightness(FfiConverterUint8INSTANCE.Lower(brightness), _uniffiStatus)
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue *Command
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterCommandINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

func CommandCharBrightness(offsetX uint64, offsetY uint64, grid *BrightnessGrid) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_char_brightness(FfiConverterUint64INSTANCE.Lower(offsetX), FfiConverterUint64INSTANCE.Lower(offsetY), FfiConverterBrightnessGridINSTANCE.Lower(grid), _uniffiStatus)
	}))
}

func CommandClear() *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_clear( _uniffiStatus)
	}))
}

func CommandClone(other *Command) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_clone(FfiConverterCommandINSTANCE.Lower(other), _uniffiStatus)
	}))
}

func CommandCp437Data(offsetX uint64, offsetY uint64, grid *Cp437Grid) *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_cp437_data(FfiConverterUint64INSTANCE.Lower(offsetX), FfiConverterUint64INSTANCE.Lower(offsetY), FfiConverterCp437GridINSTANCE.Lower(grid), _uniffiStatus)
	}))
}

func CommandFadeOut() *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_fade_out( _uniffiStatus)
	}))
}

func CommandHardReset() *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_hard_reset( _uniffiStatus)
	}))
}



func (_self *Command)Equals(other *Command) bool {
	_pointer := _self.ffiObject.incrementPointer("*Command")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_command_equals(
		_pointer,FfiConverterCommandINSTANCE.Lower(other), _uniffiStatus)
	}))
}



func (object *Command)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterCommand struct {}

var FfiConverterCommandINSTANCE = FfiConverterCommand{}

func (c FfiConverterCommand) Lift(pointer unsafe.Pointer) *Command {
	result := &Command {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_command(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*Command).Destroy)
	return result
}

func (c FfiConverterCommand) Read(reader io.Reader) *Command {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterCommand) Lower(value *Command) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Command")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterCommand) Write(writer io.Writer, value *Command) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerCommand struct {}

func (_ FfiDestroyerCommand) Destroy(value *Command) {
	value.Destroy()
}


type Connection struct {
	ffiObject FfiObject
}
func NewConnection(host string) (*Connection, error) {
	_uniffiRV, _uniffiErr := rustCallWithError(FfiConverterTypeServicePointError{},func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new(FfiConverterStringINSTANCE.Lower(host), _uniffiStatus)
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue *Connection
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterConnectionINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}


func ConnectionNewFake() *Connection {
	return FfiConverterConnectionINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new_fake( _uniffiStatus)
	}))
}



func (_self *Connection)Send(command *Command) error {
	_pointer := _self.ffiObject.incrementPointer("*Connection")
	defer _self.ffiObject.decrementPointer()
	_, _uniffiErr := rustCallWithError(FfiConverterTypeServicePointError{},func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_connection_send(
		_pointer,FfiConverterCommandINSTANCE.Lower(command), _uniffiStatus)
		return false
	})
		return _uniffiErr
}



func (object *Connection)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterConnection struct {}

var FfiConverterConnectionINSTANCE = FfiConverterConnection{}

func (c FfiConverterConnection) Lift(pointer unsafe.Pointer) *Connection {
	result := &Connection {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_connection(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*Connection).Destroy)
	return result
}

func (c FfiConverterConnection) Read(reader io.Reader) *Connection {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterConnection) Lower(value *Connection) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Connection")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterConnection) Write(writer io.Writer, value *Connection) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerConnection struct {}

func (_ FfiDestroyerConnection) Destroy(value *Connection) {
	value.Destroy()
}


type Cp437Grid struct {
	ffiObject FfiObject
}
func NewCp437Grid(width uint64, height uint64) *Cp437Grid {
	return FfiConverterCp437GridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_new(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), _uniffiStatus)
	}))
}


func Cp437GridClone(other *Cp437Grid) *Cp437Grid {
	return FfiConverterCp437GridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_clone(FfiConverterCp437GridINSTANCE.Lower(other), _uniffiStatus)
	}))
}

func Cp437GridLoad(width uint64, height uint64, data []byte) *Cp437Grid {
	return FfiConverterCp437GridINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_cp437grid_load(FfiConverterUint64INSTANCE.Lower(width), FfiConverterUint64INSTANCE.Lower(height), FfiConverterBytesINSTANCE.Lower(data), _uniffiStatus)
	}))
}



func (_self *Cp437Grid)CopyRaw() []byte {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBytesINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_copy_raw(
		_pointer, _uniffiStatus)
	}))
}


func (_self *Cp437Grid)Equals(other *Cp437Grid) bool {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterBoolINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.int8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_equals(
		_pointer,FfiConverterCp437GridINSTANCE.Lower(other), _uniffiStatus)
	}))
}


func (_self *Cp437Grid)Fill(value uint8)  {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_fill(
		_pointer,FfiConverterUint8INSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *Cp437Grid)Get(x uint64, y uint64) uint8 {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint8INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint8_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_get(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), _uniffiStatus)
	}))
}


func (_self *Cp437Grid)Height() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_height(
		_pointer, _uniffiStatus)
	}))
}


func (_self *Cp437Grid)Set(x uint64, y uint64, value uint8)  {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	rustCall(func(_uniffiStatus *C.RustCallStatus) bool {
		C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_set(
		_pointer,FfiConverterUint64INSTANCE.Lower(x), FfiConverterUint64INSTANCE.Lower(y), FfiConverterUint8INSTANCE.Lower(value), _uniffiStatus)
		return false
	})
}


func (_self *Cp437Grid)Width() uint64 {
	_pointer := _self.ffiObject.incrementPointer("*Cp437Grid")
	defer _self.ffiObject.decrementPointer()
	return FfiConverterUint64INSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint64_t {
		return C.uniffi_servicepoint_binding_uniffi_fn_method_cp437grid_width(
		_pointer, _uniffiStatus)
	}))
}



func (object *Cp437Grid)Destroy() {
	runtime.SetFinalizer(object, nil)
	object.ffiObject.destroy()
}

type FfiConverterCp437Grid struct {}

var FfiConverterCp437GridINSTANCE = FfiConverterCp437Grid{}

func (c FfiConverterCp437Grid) Lift(pointer unsafe.Pointer) *Cp437Grid {
	result := &Cp437Grid {
		newFfiObject(
			pointer,
			func(pointer unsafe.Pointer, status *C.RustCallStatus) {
				C.uniffi_servicepoint_binding_uniffi_fn_free_cp437grid(pointer, status)
		}),
	}
	runtime.SetFinalizer(result, (*Cp437Grid).Destroy)
	return result
}

func (c FfiConverterCp437Grid) Read(reader io.Reader) *Cp437Grid {
	return c.Lift(unsafe.Pointer(uintptr(readUint64(reader))))
}

func (c FfiConverterCp437Grid) Lower(value *Cp437Grid) unsafe.Pointer {
	// TODO: this is bad - all synchronization from ObjectRuntime.go is discarded here,
	// because the pointer will be decremented immediately after this function returns,
	// and someone will be left holding onto a non-locked pointer.
	pointer := value.ffiObject.incrementPointer("*Cp437Grid")
	defer value.ffiObject.decrementPointer()
	return pointer
}

func (c FfiConverterCp437Grid) Write(writer io.Writer, value *Cp437Grid) {
	writeUint64(writer, uint64(uintptr(c.Lower(value))))
}

type FfiDestroyerCp437Grid struct {}

func (_ FfiDestroyerCp437Grid) Destroy(value *Cp437Grid) {
	value.Destroy()
}



type CompressionCode uint

const (
	CompressionCodeUncompressed CompressionCode = 1
	CompressionCodeZlib CompressionCode = 2
	CompressionCodeBzip2 CompressionCode = 3
	CompressionCodeLzma CompressionCode = 4
	CompressionCodeZstd CompressionCode = 5
)

type FfiConverterTypeCompressionCode struct {}

var FfiConverterTypeCompressionCodeINSTANCE = FfiConverterTypeCompressionCode{}

func (c FfiConverterTypeCompressionCode) Lift(rb RustBufferI) CompressionCode {
	return LiftFromRustBuffer[CompressionCode](c, rb)
}

func (c FfiConverterTypeCompressionCode) Lower(value CompressionCode) RustBuffer {
	return LowerIntoRustBuffer[CompressionCode](c, value)
}
func (FfiConverterTypeCompressionCode) Read(reader io.Reader) CompressionCode {
	id := readInt32(reader)
	return CompressionCode(id)
}

func (FfiConverterTypeCompressionCode) Write(writer io.Writer, value CompressionCode) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerTypeCompressionCode struct {}

func (_ FfiDestroyerTypeCompressionCode) Destroy(value CompressionCode) {
}


type ServicePointError struct {
	err error
}

func (err ServicePointError) Error() string {
	return fmt.Sprintf("ServicePointError: %s", err.err.Error())
}

func (err ServicePointError) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrServicePointErrorIoError = fmt.Errorf("ServicePointErrorIoError")
var ErrServicePointErrorInvalidBrightness = fmt.Errorf("ServicePointErrorInvalidBrightness")

// Variant structs
type ServicePointErrorIoError struct {
	Error_ string
}
func NewServicePointErrorIoError(
	error string,
) *ServicePointError {
	return &ServicePointError{
		err: &ServicePointErrorIoError{
			Error_: error,
		},
	}
}

func (err ServicePointErrorIoError) Error() string {
	return fmt.Sprint("IoError",
		": ",
		
		"Error_=",
		err.Error_,
	)
}

func (self ServicePointErrorIoError) Is(target error) bool {
	return target == ErrServicePointErrorIoError
}
type ServicePointErrorInvalidBrightness struct {
	Value uint8
}
func NewServicePointErrorInvalidBrightness(
	value uint8,
) *ServicePointError {
	return &ServicePointError{
		err: &ServicePointErrorInvalidBrightness{
			Value: value,
		},
	}
}

func (err ServicePointErrorInvalidBrightness) Error() string {
	return fmt.Sprint("InvalidBrightness",
		": ",
		
		"Value=",
		err.Value,
	)
}

func (self ServicePointErrorInvalidBrightness) Is(target error) bool {
	return target == ErrServicePointErrorInvalidBrightness
}

type FfiConverterTypeServicePointError struct{}

var FfiConverterTypeServicePointErrorINSTANCE = FfiConverterTypeServicePointError{}

func (c FfiConverterTypeServicePointError) Lift(eb RustBufferI) error {
	return LiftFromRustBuffer[*ServicePointError](c, eb)
}

func (c FfiConverterTypeServicePointError) Lower(value *ServicePointError) RustBuffer {
	return LowerIntoRustBuffer[*ServicePointError](c, value)
}

func (c FfiConverterTypeServicePointError) Read(reader io.Reader) *ServicePointError {
	errorID := readUint32(reader)

	switch errorID {
	case 1:
		return &ServicePointError{&ServicePointErrorIoError{
			Error_: FfiConverterStringINSTANCE.Read(reader),
		}}
	case 2:
		return &ServicePointError{&ServicePointErrorInvalidBrightness{
			Value: FfiConverterUint8INSTANCE.Read(reader),
		}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterTypeServicePointError.Read()", errorID))
	}
}

func (c FfiConverterTypeServicePointError) Write(writer io.Writer, value *ServicePointError) {
	switch variantValue := value.err.(type) {
		case *ServicePointErrorIoError:
			writeInt32(writer, 1)
			FfiConverterStringINSTANCE.Write(writer, variantValue.Error_)
		case *ServicePointErrorInvalidBrightness:
			writeInt32(writer, 2)
			FfiConverterUint8INSTANCE.Write(writer, variantValue.Value)
		default:
			_ = variantValue
			panic(fmt.Sprintf("invalid error value `%v` in FfiConverterTypeServicePointError.Write", value))
	}
}

