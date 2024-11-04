// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!
import Foundation

// Depending on the consumer's build setup, the low-level FFI code
// might be in a separate module, or it might be compiled inline into
// this module. This is a bit of light hackery to work with both.
#if canImport(servicepoint_binding_uniffiFFI)
import servicepoint_binding_uniffiFFI
#endif

fileprivate extension RustBuffer {
    // Allocate a new buffer, copying the contents of a `UInt8` array.
    init(bytes: [UInt8]) {
        let rbuf = bytes.withUnsafeBufferPointer { ptr in
            RustBuffer.from(ptr)
        }
        self.init(capacity: rbuf.capacity, len: rbuf.len, data: rbuf.data)
    }

    static func from(_ ptr: UnsafeBufferPointer<UInt8>) -> RustBuffer {
        try! rustCall { ffi_servicepoint_binding_uniffi_rustbuffer_from_bytes(ForeignBytes(bufferPointer: ptr), $0) }
    }

    // Frees the buffer in place.
    // The buffer must not be used after this is called.
    func deallocate() {
        try! rustCall { ffi_servicepoint_binding_uniffi_rustbuffer_free(self, $0) }
    }
}

fileprivate extension ForeignBytes {
    init(bufferPointer: UnsafeBufferPointer<UInt8>) {
        self.init(len: Int32(bufferPointer.count), data: bufferPointer.baseAddress)
    }
}

// For every type used in the interface, we provide helper methods for conveniently
// lifting and lowering that type from C-compatible data, and for reading and writing
// values of that type in a buffer.

// Helper classes/extensions that don't change.
// Someday, this will be in a library of its own.

fileprivate extension Data {
    init(rustBuffer: RustBuffer) {
        // TODO: This copies the buffer. Can we read directly from a
        // Rust buffer?
        self.init(bytes: rustBuffer.data!, count: Int(rustBuffer.len))
    }
}

// Define reader functionality.  Normally this would be defined in a class or
// struct, but we use standalone functions instead in order to make external
// types work.
//
// With external types, one swift source file needs to be able to call the read
// method on another source file's FfiConverter, but then what visibility
// should Reader have?
// - If Reader is fileprivate, then this means the read() must also
//   be fileprivate, which doesn't work with external types.
// - If Reader is internal/public, we'll get compile errors since both source
//   files will try define the same type.
//
// Instead, the read() method and these helper functions input a tuple of data

fileprivate func createReader(data: Data) -> (data: Data, offset: Data.Index) {
    (data: data, offset: 0)
}

// Reads an integer at the current offset, in big-endian order, and advances
// the offset on success. Throws if reading the integer would move the
// offset past the end of the buffer.
fileprivate func readInt<T: FixedWidthInteger>(_ reader: inout (data: Data, offset: Data.Index)) throws -> T {
    let range = reader.offset..<reader.offset + MemoryLayout<T>.size
    guard reader.data.count >= range.upperBound else {
        throw UniffiInternalError.bufferOverflow
    }
    if T.self == UInt8.self {
        let value = reader.data[reader.offset]
        reader.offset += 1
        return value as! T
    }
    var value: T = 0
    let _ = withUnsafeMutableBytes(of: &value, { reader.data.copyBytes(to: $0, from: range)})
    reader.offset = range.upperBound
    return value.bigEndian
}

// Reads an arbitrary number of bytes, to be used to read
// raw bytes, this is useful when lifting strings
fileprivate func readBytes(_ reader: inout (data: Data, offset: Data.Index), count: Int) throws -> Array<UInt8> {
    let range = reader.offset..<(reader.offset+count)
    guard reader.data.count >= range.upperBound else {
        throw UniffiInternalError.bufferOverflow
    }
    var value = [UInt8](repeating: 0, count: count)
    value.withUnsafeMutableBufferPointer({ buffer in
        reader.data.copyBytes(to: buffer, from: range)
    })
    reader.offset = range.upperBound
    return value
}

// Reads a float at the current offset.
fileprivate func readFloat(_ reader: inout (data: Data, offset: Data.Index)) throws -> Float {
    return Float(bitPattern: try readInt(&reader))
}

// Reads a float at the current offset.
fileprivate func readDouble(_ reader: inout (data: Data, offset: Data.Index)) throws -> Double {
    return Double(bitPattern: try readInt(&reader))
}

// Indicates if the offset has reached the end of the buffer.
fileprivate func hasRemaining(_ reader: (data: Data, offset: Data.Index)) -> Bool {
    return reader.offset < reader.data.count
}

// Define writer functionality.  Normally this would be defined in a class or
// struct, but we use standalone functions instead in order to make external
// types work.  See the above discussion on Readers for details.

fileprivate func createWriter() -> [UInt8] {
    return []
}

fileprivate func writeBytes<S>(_ writer: inout [UInt8], _ byteArr: S) where S: Sequence, S.Element == UInt8 {
    writer.append(contentsOf: byteArr)
}

// Writes an integer in big-endian order.
//
// Warning: make sure what you are trying to write
// is in the correct type!
fileprivate func writeInt<T: FixedWidthInteger>(_ writer: inout [UInt8], _ value: T) {
    var value = value.bigEndian
    withUnsafeBytes(of: &value) { writer.append(contentsOf: $0) }
}

fileprivate func writeFloat(_ writer: inout [UInt8], _ value: Float) {
    writeInt(&writer, value.bitPattern)
}

fileprivate func writeDouble(_ writer: inout [UInt8], _ value: Double) {
    writeInt(&writer, value.bitPattern)
}

// Protocol for types that transfer other types across the FFI. This is
// analogous go the Rust trait of the same name.
fileprivate protocol FfiConverter {
    associatedtype FfiType
    associatedtype SwiftType

    static func lift(_ value: FfiType) throws -> SwiftType
    static func lower(_ value: SwiftType) -> FfiType
    static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> SwiftType
    static func write(_ value: SwiftType, into buf: inout [UInt8])
}

// Types conforming to `Primitive` pass themselves directly over the FFI.
fileprivate protocol FfiConverterPrimitive: FfiConverter where FfiType == SwiftType { }

extension FfiConverterPrimitive {
    public static func lift(_ value: FfiType) throws -> SwiftType {
        return value
    }

    public static func lower(_ value: SwiftType) -> FfiType {
        return value
    }
}

// Types conforming to `FfiConverterRustBuffer` lift and lower into a `RustBuffer`.
// Used for complex types where it's hard to write a custom lift/lower.
fileprivate protocol FfiConverterRustBuffer: FfiConverter where FfiType == RustBuffer {}

extension FfiConverterRustBuffer {
    public static func lift(_ buf: RustBuffer) throws -> SwiftType {
        var reader = createReader(data: Data(rustBuffer: buf))
        let value = try read(from: &reader)
        if hasRemaining(reader) {
            throw UniffiInternalError.incompleteData
        }
        buf.deallocate()
        return value
    }

    public static func lower(_ value: SwiftType) -> RustBuffer {
          var writer = createWriter()
          write(value, into: &writer)
          return RustBuffer(bytes: writer)
    }
}
// An error type for FFI errors. These errors occur at the UniFFI level, not
// the library level.
fileprivate enum UniffiInternalError: LocalizedError {
    case bufferOverflow
    case incompleteData
    case unexpectedOptionalTag
    case unexpectedEnumCase
    case unexpectedNullPointer
    case unexpectedRustCallStatusCode
    case unexpectedRustCallError
    case unexpectedStaleHandle
    case rustPanic(_ message: String)

    public var errorDescription: String? {
        switch self {
        case .bufferOverflow: return "Reading the requested value would read past the end of the buffer"
        case .incompleteData: return "The buffer still has data after lifting its containing value"
        case .unexpectedOptionalTag: return "Unexpected optional tag; should be 0 or 1"
        case .unexpectedEnumCase: return "Raw enum value doesn't match any cases"
        case .unexpectedNullPointer: return "Raw pointer value was null"
        case .unexpectedRustCallStatusCode: return "Unexpected RustCallStatus code"
        case .unexpectedRustCallError: return "CALL_ERROR but no errorClass specified"
        case .unexpectedStaleHandle: return "The object in the handle map has been dropped already"
        case let .rustPanic(message): return message
        }
    }
}

fileprivate let CALL_SUCCESS: Int8 = 0
fileprivate let CALL_ERROR: Int8 = 1
fileprivate let CALL_PANIC: Int8 = 2
fileprivate let CALL_CANCELLED: Int8 = 3

fileprivate extension RustCallStatus {
    init() {
        self.init(
            code: CALL_SUCCESS,
            errorBuf: RustBuffer.init(
                capacity: 0,
                len: 0,
                data: nil
            )
        )
    }
}

private func rustCall<T>(_ callback: (UnsafeMutablePointer<RustCallStatus>) -> T) throws -> T {
    try makeRustCall(callback, errorHandler: nil)
}

private func rustCallWithError<T>(
    _ errorHandler: @escaping (RustBuffer) throws -> Error,
    _ callback: (UnsafeMutablePointer<RustCallStatus>) -> T) throws -> T {
    try makeRustCall(callback, errorHandler: errorHandler)
}

private func makeRustCall<T>(
    _ callback: (UnsafeMutablePointer<RustCallStatus>) -> T,
    errorHandler: ((RustBuffer) throws -> Error)?
) throws -> T {
    uniffiEnsureInitialized()
    var callStatus = RustCallStatus.init()
    let returnedVal = callback(&callStatus)
    try uniffiCheckCallStatus(callStatus: callStatus, errorHandler: errorHandler)
    return returnedVal
}

private func uniffiCheckCallStatus(
    callStatus: RustCallStatus,
    errorHandler: ((RustBuffer) throws -> Error)?
) throws {
    switch callStatus.code {
        case CALL_SUCCESS:
            return

        case CALL_ERROR:
            if let errorHandler = errorHandler {
                throw try errorHandler(callStatus.errorBuf)
            } else {
                callStatus.errorBuf.deallocate()
                throw UniffiInternalError.unexpectedRustCallError
            }

        case CALL_PANIC:
            // When the rust code sees a panic, it tries to construct a RustBuffer
            // with the message.  But if that code panics, then it just sends back
            // an empty buffer.
            if callStatus.errorBuf.len > 0 {
                throw UniffiInternalError.rustPanic(try FfiConverterString.lift(callStatus.errorBuf))
            } else {
                callStatus.errorBuf.deallocate()
                throw UniffiInternalError.rustPanic("Rust panic")
            }

        case CALL_CANCELLED:
                throw CancellationError()

        default:
            throw UniffiInternalError.unexpectedRustCallStatusCode
    }
}

// Public interface members begin here.


fileprivate struct FfiConverterUInt8: FfiConverterPrimitive {
    typealias FfiType = UInt8
    typealias SwiftType = UInt8

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> UInt8 {
        return try lift(readInt(&buf))
    }

    public static func write(_ value: UInt8, into buf: inout [UInt8]) {
        writeInt(&buf, lower(value))
    }
}

fileprivate struct FfiConverterUInt64: FfiConverterPrimitive {
    typealias FfiType = UInt64
    typealias SwiftType = UInt64

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> UInt64 {
        return try lift(readInt(&buf))
    }

    public static func write(_ value: SwiftType, into buf: inout [UInt8]) {
        writeInt(&buf, lower(value))
    }
}

fileprivate struct FfiConverterBool : FfiConverter {
    typealias FfiType = Int8
    typealias SwiftType = Bool

    public static func lift(_ value: Int8) throws -> Bool {
        return value != 0
    }

    public static func lower(_ value: Bool) -> Int8 {
        return value ? 1 : 0
    }

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Bool {
        return try lift(readInt(&buf))
    }

    public static func write(_ value: Bool, into buf: inout [UInt8]) {
        writeInt(&buf, lower(value))
    }
}

fileprivate struct FfiConverterString: FfiConverter {
    typealias SwiftType = String
    typealias FfiType = RustBuffer

    public static func lift(_ value: RustBuffer) throws -> String {
        defer {
            value.deallocate()
        }
        if value.data == nil {
            return String()
        }
        let bytes = UnsafeBufferPointer<UInt8>(start: value.data!, count: Int(value.len))
        return String(bytes: bytes, encoding: String.Encoding.utf8)!
    }

    public static func lower(_ value: String) -> RustBuffer {
        return value.utf8CString.withUnsafeBufferPointer { ptr in
            // The swift string gives us int8_t, we want uint8_t.
            ptr.withMemoryRebound(to: UInt8.self) { ptr in
                // The swift string gives us a trailing null byte, we don't want it.
                let buf = UnsafeBufferPointer(rebasing: ptr.prefix(upTo: ptr.count - 1))
                return RustBuffer.from(buf)
            }
        }
    }

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> String {
        let len: Int32 = try readInt(&buf)
        return String(bytes: try readBytes(&buf, count: Int(len)), encoding: String.Encoding.utf8)!
    }

    public static func write(_ value: String, into buf: inout [UInt8]) {
        let len = Int32(value.utf8.count)
        writeInt(&buf, len)
        writeBytes(&buf, value.utf8)
    }
}

fileprivate struct FfiConverterData: FfiConverterRustBuffer {
    typealias SwiftType = Data

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Data {
        let len: Int32 = try readInt(&buf)
        return Data(try readBytes(&buf, count: Int(len)))
    }

    public static func write(_ value: Data, into buf: inout [UInt8]) {
        let len = Int32(value.count)
        writeInt(&buf, len)
        writeBytes(&buf, value)
    }
}


public protocol BitVecProtocol {
    func fill(value: Bool)  
    func get(index: UInt64)   -> Bool
    func len()   -> UInt64
    func set(index: UInt64, value: Bool)  
    
}

public class BitVec: BitVecProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `FfiConverter` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
    public convenience init(size: UInt64)  {
        self.init(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_new(
        FfiConverterUInt64.lower(size),$0)
})
    }

    deinit {
        try! rustCall { uniffi_servicepoint_binding_uniffi_fn_free_bitvec(pointer, $0) }
    }

    

    public static func load(data: Data)  -> BitVec {
        return BitVec(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_bitvec_load(
        FfiConverterData.lower(data),$0)
})
    }

    

    
    

    public func fill(value: Bool)  {
        try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitvec_fill(self.pointer, 
        FfiConverterBool.lower(value),$0
    )
}
    }

    public func get(index: UInt64)  -> Bool {
        return try!  FfiConverterBool.lift(
            try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitvec_get(self.pointer, 
        FfiConverterUInt64.lower(index),$0
    )
}
        )
    }

    public func len()  -> UInt64 {
        return try!  FfiConverterUInt64.lift(
            try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitvec_len(self.pointer, $0
    )
}
        )
    }

    public func set(index: UInt64, value: Bool)  {
        try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitvec_set(self.pointer, 
        FfiConverterUInt64.lower(index),
        FfiConverterBool.lower(value),$0
    )
}
    }
}

public struct FfiConverterTypeBitVec: FfiConverter {
    typealias FfiType = UnsafeMutableRawPointer
    typealias SwiftType = BitVec

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> BitVec {
        let v: UInt64 = try readInt(&buf)
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try lift(ptr!)
    }

    public static func write(_ value: BitVec, into buf: inout [UInt8]) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        writeInt(&buf, UInt64(bitPattern: Int64(Int(bitPattern: lower(value)))))
    }

    public static func lift(_ pointer: UnsafeMutableRawPointer) throws -> BitVec {
        return BitVec(unsafeFromRawPointer: pointer)
    }

    public static func lower(_ value: BitVec) -> UnsafeMutableRawPointer {
        return value.pointer
    }
}


public func FfiConverterTypeBitVec_lift(_ pointer: UnsafeMutableRawPointer) throws -> BitVec {
    return try FfiConverterTypeBitVec.lift(pointer)
}

public func FfiConverterTypeBitVec_lower(_ value: BitVec) -> UnsafeMutableRawPointer {
    return FfiConverterTypeBitVec.lower(value)
}


public protocol BitmapProtocol {
    func fill(value: Bool)  
    func get(x: UInt64, y: UInt64)   -> Bool
    func height()   -> UInt64
    func set(x: UInt64, y: UInt64, value: Bool)  
    func width()   -> UInt64
    
}

public class Bitmap: BitmapProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `FfiConverter` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
    public convenience init(width: UInt64, height: UInt64)  {
        self.init(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new(
        FfiConverterUInt64.lower(width),
        FfiConverterUInt64.lower(height),$0)
})
    }

    deinit {
        try! rustCall { uniffi_servicepoint_binding_uniffi_fn_free_bitmap(pointer, $0) }
    }

    

    public static func load(width: UInt64, height: UInt64, data: Data)  -> Bitmap {
        return Bitmap(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_load(
        FfiConverterUInt64.lower(width),
        FfiConverterUInt64.lower(height),
        FfiConverterData.lower(data),$0)
})
    }

    

    public static func newMaxSized()  -> Bitmap {
        return Bitmap(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_bitmap_new_max_sized($0)
})
    }

    

    
    

    public func fill(value: Bool)  {
        try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitmap_fill(self.pointer, 
        FfiConverterBool.lower(value),$0
    )
}
    }

    public func get(x: UInt64, y: UInt64)  -> Bool {
        return try!  FfiConverterBool.lift(
            try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitmap_get(self.pointer, 
        FfiConverterUInt64.lower(x),
        FfiConverterUInt64.lower(y),$0
    )
}
        )
    }

    public func height()  -> UInt64 {
        return try!  FfiConverterUInt64.lift(
            try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitmap_height(self.pointer, $0
    )
}
        )
    }

    public func set(x: UInt64, y: UInt64, value: Bool)  {
        try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitmap_set(self.pointer, 
        FfiConverterUInt64.lower(x),
        FfiConverterUInt64.lower(y),
        FfiConverterBool.lower(value),$0
    )
}
    }

    public func width()  -> UInt64 {
        return try!  FfiConverterUInt64.lift(
            try! 
    rustCall() {
    
    uniffi_servicepoint_binding_uniffi_fn_method_bitmap_width(self.pointer, $0
    )
}
        )
    }
}

public struct FfiConverterTypeBitmap: FfiConverter {
    typealias FfiType = UnsafeMutableRawPointer
    typealias SwiftType = Bitmap

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Bitmap {
        let v: UInt64 = try readInt(&buf)
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try lift(ptr!)
    }

    public static func write(_ value: Bitmap, into buf: inout [UInt8]) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        writeInt(&buf, UInt64(bitPattern: Int64(Int(bitPattern: lower(value)))))
    }

    public static func lift(_ pointer: UnsafeMutableRawPointer) throws -> Bitmap {
        return Bitmap(unsafeFromRawPointer: pointer)
    }

    public static func lower(_ value: Bitmap) -> UnsafeMutableRawPointer {
        return value.pointer
    }
}


public func FfiConverterTypeBitmap_lift(_ pointer: UnsafeMutableRawPointer) throws -> Bitmap {
    return try FfiConverterTypeBitmap.lift(pointer)
}

public func FfiConverterTypeBitmap_lower(_ value: Bitmap) -> UnsafeMutableRawPointer {
    return FfiConverterTypeBitmap.lower(value)
}


public protocol CommandProtocol {
    
}

public class Command: CommandProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `FfiConverter` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }

    deinit {
        try! rustCall { uniffi_servicepoint_binding_uniffi_fn_free_command(pointer, $0) }
    }

    

    public static func bitmapLinearWin(offsetX: UInt64, offsetY: UInt64, bitmap: Bitmap)  -> Command {
        return Command(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_command_bitmap_linear_win(
        FfiConverterUInt64.lower(offsetX),
        FfiConverterUInt64.lower(offsetY),
        FfiConverterTypeBitmap.lower(bitmap),$0)
})
    }

    

    public static func brightness(brightness: UInt8) throws -> Command {
        return Command(unsafeFromRawPointer: try rustCallWithError(FfiConverterTypeServicePointError.lift) {
    uniffi_servicepoint_binding_uniffi_fn_constructor_command_brightness(
        FfiConverterUInt8.lower(brightness),$0)
})
    }

    

    public static func clear()  -> Command {
        return Command(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_command_clear($0)
})
    }

    

    public static func fadeOut()  -> Command {
        return Command(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_command_fade_out($0)
})
    }

    

    public static func hardReset()  -> Command {
        return Command(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_command_hard_reset($0)
})
    }

    

    
    
}

public struct FfiConverterTypeCommand: FfiConverter {
    typealias FfiType = UnsafeMutableRawPointer
    typealias SwiftType = Command

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Command {
        let v: UInt64 = try readInt(&buf)
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try lift(ptr!)
    }

    public static func write(_ value: Command, into buf: inout [UInt8]) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        writeInt(&buf, UInt64(bitPattern: Int64(Int(bitPattern: lower(value)))))
    }

    public static func lift(_ pointer: UnsafeMutableRawPointer) throws -> Command {
        return Command(unsafeFromRawPointer: pointer)
    }

    public static func lower(_ value: Command) -> UnsafeMutableRawPointer {
        return value.pointer
    }
}


public func FfiConverterTypeCommand_lift(_ pointer: UnsafeMutableRawPointer) throws -> Command {
    return try FfiConverterTypeCommand.lift(pointer)
}

public func FfiConverterTypeCommand_lower(_ value: Command) -> UnsafeMutableRawPointer {
    return FfiConverterTypeCommand.lower(value)
}


public protocol ConnectionProtocol {
    func send(command: Command)  throws
    
}

public class Connection: ConnectionProtocol {
    fileprivate let pointer: UnsafeMutableRawPointer

    // TODO: We'd like this to be `private` but for Swifty reasons,
    // we can't implement `FfiConverter` without making this `required` and we can't
    // make it `required` without making it `public`.
    required init(unsafeFromRawPointer pointer: UnsafeMutableRawPointer) {
        self.pointer = pointer
    }
    public convenience init(host: String) throws {
        self.init(unsafeFromRawPointer: try rustCallWithError(FfiConverterTypeServicePointError.lift) {
    uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new(
        FfiConverterString.lower(host),$0)
})
    }

    deinit {
        try! rustCall { uniffi_servicepoint_binding_uniffi_fn_free_connection(pointer, $0) }
    }

    

    public static func newFake()  -> Connection {
        return Connection(unsafeFromRawPointer: try! rustCall() {
    uniffi_servicepoint_binding_uniffi_fn_constructor_connection_new_fake($0)
})
    }

    

    
    

    public func send(command: Command) throws {
        try 
    rustCallWithError(FfiConverterTypeServicePointError.lift) {
    uniffi_servicepoint_binding_uniffi_fn_method_connection_send(self.pointer, 
        FfiConverterTypeCommand.lower(command),$0
    )
}
    }
}

public struct FfiConverterTypeConnection: FfiConverter {
    typealias FfiType = UnsafeMutableRawPointer
    typealias SwiftType = Connection

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Connection {
        let v: UInt64 = try readInt(&buf)
        // The Rust code won't compile if a pointer won't fit in a UInt64.
        // We have to go via `UInt` because that's the thing that's the size of a pointer.
        let ptr = UnsafeMutableRawPointer(bitPattern: UInt(truncatingIfNeeded: v))
        if (ptr == nil) {
            throw UniffiInternalError.unexpectedNullPointer
        }
        return try lift(ptr!)
    }

    public static func write(_ value: Connection, into buf: inout [UInt8]) {
        // This fiddling is because `Int` is the thing that's the same size as a pointer.
        // The Rust code won't compile if a pointer won't fit in a `UInt64`.
        writeInt(&buf, UInt64(bitPattern: Int64(Int(bitPattern: lower(value)))))
    }

    public static func lift(_ pointer: UnsafeMutableRawPointer) throws -> Connection {
        return Connection(unsafeFromRawPointer: pointer)
    }

    public static func lower(_ value: Connection) -> UnsafeMutableRawPointer {
        return value.pointer
    }
}


public func FfiConverterTypeConnection_lift(_ pointer: UnsafeMutableRawPointer) throws -> Connection {
    return try FfiConverterTypeConnection.lift(pointer)
}

public func FfiConverterTypeConnection_lower(_ value: Connection) -> UnsafeMutableRawPointer {
    return FfiConverterTypeConnection.lower(value)
}

public enum ServicePointError {

    
    
    case IoError(error: String)
    case InvalidBrightness(value: UInt8)

    fileprivate static func uniffiErrorHandler(_ error: RustBuffer) throws -> Error {
        return try FfiConverterTypeServicePointError.lift(error)
    }
}


public struct FfiConverterTypeServicePointError: FfiConverterRustBuffer {
    typealias SwiftType = ServicePointError

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> ServicePointError {
        let variant: Int32 = try readInt(&buf)
        switch variant {

        

        
        case 1: return .IoError(
            error: try FfiConverterString.read(from: &buf)
            )
        case 2: return .InvalidBrightness(
            value: try FfiConverterUInt8.read(from: &buf)
            )

         default: throw UniffiInternalError.unexpectedEnumCase
        }
    }

    public static func write(_ value: ServicePointError, into buf: inout [UInt8]) {
        switch value {

        

        
        
        case let .IoError(error):
            writeInt(&buf, Int32(1))
            FfiConverterString.write(error, into: &buf)
            
        
        case let .InvalidBrightness(value):
            writeInt(&buf, Int32(2))
            FfiConverterUInt8.write(value, into: &buf)
            
        }
    }
}


extension ServicePointError: Equatable, Hashable {}

extension ServicePointError: Error { }

private enum InitializationResult {
    case ok
    case contractVersionMismatch
    case apiChecksumMismatch
}
// Use a global variables to perform the versioning checks. Swift ensures that
// the code inside is only computed once.
private var initializationResult: InitializationResult {
    // Get the bindings contract version from our ComponentInterface
    let bindings_contract_version = 24
    // Get the scaffolding contract version by calling the into the dylib
    let scaffolding_contract_version = ffi_servicepoint_binding_uniffi_uniffi_contract_version()
    if bindings_contract_version != scaffolding_contract_version {
        return InitializationResult.contractVersionMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_fill() != 12255) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_get() != 43835) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_len() != 22196) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitvec_set() != 16307) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_fill() != 43887) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_get() != 61136) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_height() != 44991) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_set() != 25290) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_bitmap_width() != 30837) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_method_connection_send() != 23796) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_load() != 48913) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_bitvec_new() != 11865) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_load() != 24109) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new() != 49832) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_bitmap_new_max_sized() != 63762) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_command_bitmap_linear_win() != 51700) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_command_brightness() != 11291) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_command_clear() != 11035) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_command_fade_out() != 49231) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_command_hard_reset() != 62130) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new() != 30445) {
        return InitializationResult.apiChecksumMismatch
    }
    if (uniffi_servicepoint_binding_uniffi_checksum_constructor_connection_new_fake() != 54331) {
        return InitializationResult.apiChecksumMismatch
    }

    return InitializationResult.ok
}

private func uniffiEnsureInitialized() {
    switch initializationResult {
    case .ok:
        break
    case .contractVersionMismatch:
        fatalError("UniFFI contract version mismatch: try cleaning and rebuilding your project")
    case .apiChecksumMismatch:
        fatalError("UniFFI API checksum mismatch: try cleaning and rebuilding your project")
    }
}