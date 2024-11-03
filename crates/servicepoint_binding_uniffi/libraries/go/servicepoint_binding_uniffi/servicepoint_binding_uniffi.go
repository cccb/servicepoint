
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
		return C.uniffi_servicepoint_binding_uniffi_checksum_method_connection_send(uniffiStatus)
	})
	if checksum != 23796 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_method_connection_send: UniFFI API checksum mismatch")
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
		return C.uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear(uniffiStatus)
	})
	if checksum != 11035 {
		// If this happens try cleaning and rebuilding your project
		panic("servicepoint_binding_uniffi: uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear: UniFFI API checksum mismatch")
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
type Command struct {
	ffiObject FfiObject
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

func CommandClear() *Command {
	return FfiConverterCommandINSTANCE.Lift(rustCall(func(_uniffiStatus *C.RustCallStatus) unsafe.Pointer {
		return C.uniffi_servicepoint_binding_uniffi_fn_constructor_command_clear( _uniffiStatus)
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

