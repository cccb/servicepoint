// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace ServicePoint.BindGen
{
    public static unsafe partial class NativeMethods
    {
        const string __DllName = "servicepoint_binding_c";

        public const nuint TILE_SIZE = 8;
        public const nuint TILE_WIDTH = 56;
        public const nuint TILE_HEIGHT = 20;


        /// <summary>Creates a new `BitVec` instance.  # Arguments  * `size`: size in bits.  returns: `BitVec` with all bits set to false.  # Panics  When `size` is not divisible by 8.  # Safety  The caller has to make sure that:  - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_bit_vec_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBitVec* sp_bit_vec_new(nuint size);

        /// <summary>Interpret the data as a series of bits and load then into a new `BitVec` instance.  # Safety  The caller has to make sure that:  - `data` points to a valid memory location of at least `data_length` bytes in size. - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_bit_vec_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBitVec* sp_bit_vec_load(byte* data, nuint data_length);

        /// <summary>Clones a `BitVec`.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to concurrently - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_bit_vec_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBitVec* sp_bit_vec_clone(CBitVec* @this);

        /// <summary>Deallocates a `BitVec`.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not used concurrently or after this call - `this` was not passed to another consuming function, e.g. to create a `Command`</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_bit_vec_dealloc(CBitVec* @this);

        /// <summary>Gets the value of a bit from the `BitVec`.  # Arguments  * `this`: instance to read from * `index`: the bit index to read  returns: value of the bit  # Panics  When accessing `index` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_get", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool sp_bit_vec_get(CBitVec* @this, nuint index);

        /// <summary>Sets the value of a bit in the `BitVec`.  # Arguments  * `this`: instance to write to * `index`: the bit index to edit * `value`: the value to set the bit to  returns: old value of the bit  # Panics  When accessing `index` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_set", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_bit_vec_set(CBitVec* @this, nuint index, [MarshalAs(UnmanagedType.U1)] bool value);

        /// <summary>Sets the value of all bits in the `BitVec`.  # Arguments  * `value`: the value to set all bits to  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_fill", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_bit_vec_fill(CBitVec* @this, [MarshalAs(UnmanagedType.U1)] bool value);

        /// <summary>Gets the length of the `BitVec` in bits.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec`</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_len", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_bit_vec_len(CBitVec* @this);

        /// <summary>Returns true if length is 0.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec`</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_is_empty", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool sp_bit_vec_is_empty(CBitVec* @this);

        /// <summary>Gets an unsafe reference to the data of the `BitVec` instance.  ## Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - the returned memory range is never accessed after the passed `BitVec` has been freed - the returned memory range is never accessed concurrently, either via the `BitVec` or directly</summary>
        [DllImport(__DllName, EntryPoint = "sp_bit_vec_unsafe_data_ref", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CByteSlice sp_bit_vec_unsafe_data_ref(CBitVec* @this);

        /// <summary>Creates a new `BrightnessGrid` with the specified dimensions.  returns: `BrightnessGrid` initialized to 0.  # Safety  The caller has to make sure that:  - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_brightness_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBrightnessGrid* sp_brightness_grid_new(nuint width, nuint height);

        /// <summary>Loads a `BrightnessGrid` with the specified dimensions from the provided data.  # Panics  When the provided `data_length` is not sufficient for the `height` and `width`  # Safety  The caller has to make sure that:  - `data` points to a valid memory location of at least `data_length` bytes in size. - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_brightness_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBrightnessGrid* sp_brightness_grid_load(nuint width, nuint height, byte* data, nuint data_length);

        /// <summary>Clones a `BrightnessGrid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid` - `this` is not written to concurrently - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_brightness_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CBrightnessGrid* sp_brightness_grid_clone(CBrightnessGrid* @this);

        /// <summary>Deallocates a `BrightnessGrid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid` - `this` is not used concurrently or after this call - `this` was not passed to another consuming function, e.g. to create a `Command`</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_brightness_grid_dealloc(CBrightnessGrid* @this);

        /// <summary>Gets the current value at the specified position.  # Arguments  * `this`: instance to read from * `x` and `y`: position of the cell to read  # Panics  When accessing `x` or `y` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid` - `this` is not written to concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_get", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte sp_brightness_grid_get(CBrightnessGrid* @this, nuint x, nuint y);

        /// <summary>Sets the value of the specified position in the `BrightnessGrid`.  # Arguments  - `this`: instance to write to - `x` and `y`: position of the cell - `value`: the value to write to the cell  returns: old value of the cell  # Panics  - When accessing `x` or `y` out of bounds. - When providing an invalid brightness value  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_set", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_brightness_grid_set(CBrightnessGrid* @this, nuint x, nuint y, byte value);

        /// <summary>Sets the value of all cells in the `BrightnessGrid`.  # Arguments  * `this`: instance to write to * `value`: the value to set all cells to  # Panics  - When providing an invalid brightness value  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_fill", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_brightness_grid_fill(CBrightnessGrid* @this, byte value);

        /// <summary>Gets the width of the `BrightnessGrid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_width", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_brightness_grid_width(CBrightnessGrid* @this);

        /// <summary>Gets the height of the `BrightnessGrid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_height", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_brightness_grid_height(CBrightnessGrid* @this);

        /// <summary>Gets an unsafe reference to the data of the `BrightnessGrid` instance.  ## Safety  The caller has to make sure that:  - `this` points to a valid `BrightnessGrid` - the returned memory range is never accessed after the passed `BrightnessGrid` has been freed - the returned memory range is never accessed concurrently, either via the `BrightnessGrid` or directly</summary>
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_unsafe_data_ref", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CByteSlice sp_brightness_grid_unsafe_data_ref(CBrightnessGrid* @this);

        /// <summary>Creates a new `Cp437Grid` with the specified dimensions.  returns: `Cp437Grid` initialized to 0.  # Safety  The caller has to make sure that:  - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_cp437_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CCp437Grid* sp_cp437_grid_new(nuint width, nuint height);

        /// <summary>Loads a `Cp437Grid` with the specified dimensions from the provided data.  # Panics  When the provided `data_length` is not sufficient for the `height` and `width`  # Safety  The caller has to make sure that:  - `data` points to a valid memory location of at least `data_length` bytes in size. - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_cp437_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CCp437Grid* sp_cp437_grid_load(nuint width, nuint height, byte* data, nuint data_length);

        /// <summary>Clones a `Cp437Grid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid` - `this` is not written to concurrently - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_cp437_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CCp437Grid* sp_cp437_grid_clone(CCp437Grid* @this);

        /// <summary>Deallocates a `Cp437Grid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid` - `this` is not used concurrently or after this call - `this` was not passed to another consuming function, e.g. to create a `Command`</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_cp437_grid_dealloc(CCp437Grid* @this);

        /// <summary>Gets the current value at the specified position.  # Arguments  * `this`: instance to read from * `x` and `y`: position of the cell to read  # Panics  When accessing `x` or `y` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid` - `this` is not written to concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_get", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte sp_cp437_grid_get(CCp437Grid* @this, nuint x, nuint y);

        /// <summary>Sets the value of the specified position in the `Cp437Grid`.  # Arguments  * `this`: instance to write to * `x` and `y`: position of the cell * `value`: the value to write to the cell  returns: old value of the cell  # Panics  When accessing `x` or `y` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `BitVec` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_set", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_cp437_grid_set(CCp437Grid* @this, nuint x, nuint y, byte value);

        /// <summary>Sets the value of all cells in the `Cp437Grid`.  # Arguments  * `this`: instance to write to * `value`: the value to set all cells to  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_fill", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_cp437_grid_fill(CCp437Grid* @this, byte value);

        /// <summary>Gets the width of the `Cp437Grid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_width", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_cp437_grid_width(CCp437Grid* @this);

        /// <summary>Gets the height of the `Cp437Grid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_height", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_cp437_grid_height(CCp437Grid* @this);

        /// <summary>Gets an unsafe reference to the data of the `Cp437Grid` instance.  ## Safety  The caller has to make sure that:  - `this` points to a valid `Cp437Grid` - the returned memory range is never accessed after the passed `Cp437Grid` has been freed - the returned memory range is never accessed concurrently, either via the `Cp437Grid` or directly</summary>
        [DllImport(__DllName, EntryPoint = "sp_cp437_grid_unsafe_data_ref", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CByteSlice sp_cp437_grid_unsafe_data_ref(CCp437Grid* @this);

        /// <summary>Tries to turn a `Packet` into a `Command`. The packet is deallocated in the process.  Returns: pointer to new `Command` instance or NULL  # Safety  The caller has to make sure that:  - `packet` points to a valid instance of `Packet` - `packet` is not used concurrently or after this call - the result is checked for NULL - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_try_from_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_try_from_packet(Packet* packet);

        /// <summary>Clones a `Command` instance.  # Safety  The caller has to make sure that:  - `this` points to a valid instance of `Command` - `this` is not written to concurrently - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_clone(Command* original);

        /// <summary>Allocates a new `Command::Clear` instance.  # Safety  The caller has to make sure that:  - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_clear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_clear();

        /// <summary>Allocates a new `Command::HardReset` instance.  # Safety  The caller has to make sure that:  - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_hard_reset", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_hard_reset();

        /// <summary>Allocates a new `Command::FadeOut` instance.  # Safety  The caller has to make sure that:  - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_fade_out", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_fade_out();

        /// <summary>Allocates a new `Command::Brightness` instance for setting the brightness of all tiles to the same value.  # Panics  - When the provided brightness value is out of range (0-11).  # Safety  The caller has to make sure that:  - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_brightness(byte brightness);

        /// <summary>Allocates a new `Command::CharBrightness` instance. The passed `ByteGrid` gets consumed.  # Safety  The caller has to make sure that:  - `byte_grid` points to a valid instance of `ByteGrid` - `byte_grid` is not used concurrently or after this call - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_char_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_char_brightness(nuint x, nuint y, CBrightnessGrid* byte_grid);

        /// <summary>Allocates a new `Command::BitmapLinear` instance. The passed `BitVec` gets consumed.  # Safety  The caller has to make sure that:  - `bit_vec` points to a valid instance of `BitVec` - `bit_vec` is not used concurrently or after this call - `compression` matches one of the allowed enum values - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_bitmap_linear(nuint offset, CBitVec* bit_vec, CompressionCode compression);

        /// <summary>Allocates a new `Command::BitmapLinearAnd` instance. The passed `BitVec` gets consumed.  # Safety  The caller has to make sure that:  - `bit_vec` points to a valid instance of `BitVec` - `bit_vec` is not used concurrently or after this call - `compression` matches one of the allowed enum values - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_and", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_bitmap_linear_and(nuint offset, CBitVec* bit_vec, CompressionCode compression);

        /// <summary>Allocates a new `Command::BitmapLinearOr` instance. The passed `BitVec` gets consumed.  # Safety  The caller has to make sure that:  - `bit_vec` points to a valid instance of `BitVec` - `bit_vec` is not used concurrently or after this call - `compression` matches one of the allowed enum values - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_or", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_bitmap_linear_or(nuint offset, CBitVec* bit_vec, CompressionCode compression);

        /// <summary>Allocates a new `Command::BitmapLinearXor` instance. The passed `BitVec` gets consumed.  # Safety  The caller has to make sure that:  - `bit_vec` points to a valid instance of `BitVec` - `bit_vec` is not used concurrently or after this call - `compression` matches one of the allowed enum values - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_xor", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_bitmap_linear_xor(nuint offset, CBitVec* bit_vec, CompressionCode compression);

        /// <summary>Allocates a new `Command::Cp437Data` instance. The passed `ByteGrid` gets consumed.  # Safety  The caller has to make sure that:  - `byte_grid` points to a valid instance of `ByteGrid` - `byte_grid` is not used concurrently or after this call - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_cp437_data", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_cp437_data(nuint x, nuint y, CCp437Grid* byte_grid);

        /// <summary>Allocates a new `Command::BitmapLinearWin` instance. The passed `PixelGrid` gets consumed.  # Safety  The caller has to make sure that:  - `pixel_grid` points to a valid instance of `PixelGrid` - `pixel_grid` is not used concurrently or after this call - `compression` matches one of the allowed enum values - the returned `Command` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_command_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_win", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Command* sp_command_bitmap_linear_win(nuint x, nuint y, PixelGrid* pixel_grid, CompressionCode compression_code);

        /// <summary>Deallocates a `Command`.  # Safety  The caller has to make sure that:  - `this` points to a valid `Command` - `this` is not used concurrently or after this call - `this` was not passed to another consuming function, e.g. to create a `Packet`</summary>
        [DllImport(__DllName, EntryPoint = "sp_command_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_command_dealloc(Command* ptr);

        /// <summary>Creates a new instance of `Connection`.  returns: NULL if connection fails, or connected instance  # Panics  Bad string encoding  # Safety  The caller has to make sure that:  - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_connection_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_connection_open", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Connection* sp_connection_open(byte* host);

        /// <summary>Sends a `Packet` to the display using the `Connection`. The passed `Packet` gets consumed.  returns: true in case of success  # Safety  The caller has to make sure that:  - `connection` points to a valid instance of `Connection` - `packet` points to a valid instance of `Packet` - `packet` is not used concurrently or after this call</summary>
        [DllImport(__DllName, EntryPoint = "sp_connection_send", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool sp_connection_send(Connection* connection, Packet* packet);

        /// <summary>Closes and deallocates a `Connection`.  # Safety  The caller has to make sure that:  - `this` points to a valid `Connection` - `this` is not used concurrently or after this call</summary>
        [DllImport(__DllName, EntryPoint = "sp_connection_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_connection_dealloc(Connection* ptr);

        /// <summary>Creates a new `PixelGrid` with the specified dimensions.  # Arguments  * `width`: size in pixels in x-direction * `height`: size in pixels in y-direction  returns: `PixelGrid` initialized to all pixels off  # Panics  - when the width is not dividable by 8  # Safety  The caller has to make sure that:  - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_pixel_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PixelGrid* sp_pixel_grid_new(nuint width, nuint height);

        /// <summary>Loads a `PixelGrid` with the specified dimensions from the provided data.  # Arguments  * `width`: size in pixels in x-direction * `height`: size in pixels in y-direction  returns: `PixelGrid` that contains a copy of the provided data  # Panics  - when the dimensions and data size do not match exactly. - when the width is not dividable by 8  # Safety  The caller has to make sure that:  - `data` points to a valid memory location of at least `data_length` bytes in size. - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_pixel_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PixelGrid* sp_pixel_grid_load(nuint width, nuint height, byte* data, nuint data_length);

        /// <summary>Clones a `PixelGrid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - `this` is not written to concurrently - the returned instance is freed in some way, either by using a consuming function or by explicitly calling `sp_pixel_grid_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PixelGrid* sp_pixel_grid_clone(PixelGrid* @this);

        /// <summary>Deallocates a `PixelGrid`.  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - `this` is not used concurrently or after this call - `this` was not passed to another consuming function, e.g. to create a `Command`</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_pixel_grid_dealloc(PixelGrid* @this);

        /// <summary>Gets the current value at the specified position in the `PixelGrid`.  # Arguments  * `this`: instance to read from * `x` and `y`: position of the cell to read  # Panics  When accessing `x` or `y` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - `this` is not written to concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_get", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool sp_pixel_grid_get(PixelGrid* @this, nuint x, nuint y);

        /// <summary>Sets the value of the specified position in the `PixelGrid`.  # Arguments  * `this`: instance to write to * `x` and `y`: position of the cell * `value`: the value to write to the cell  returns: old value of the cell  # Panics  When accessing `x` or `y` out of bounds.  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_set", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_pixel_grid_set(PixelGrid* @this, nuint x, nuint y, [MarshalAs(UnmanagedType.U1)] bool value);

        /// <summary>Sets the state of all pixels in the `PixelGrid`.  # Arguments  * `this`: instance to write to * `value`: the value to set all pixels to  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - `this` is not written to or read from concurrently</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_fill", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_pixel_grid_fill(PixelGrid* @this, [MarshalAs(UnmanagedType.U1)] bool value);

        /// <summary>Gets the width in pixels of the `PixelGrid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_width", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_pixel_grid_width(PixelGrid* @this);

        /// <summary>Gets the height in pixels of the `PixelGrid` instance.  # Arguments  * `this`: instance to read from  # Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid`</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_height", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern nuint sp_pixel_grid_height(PixelGrid* @this);

        /// <summary>Gets an unsafe reference to the data of the `PixelGrid` instance.  ## Safety  The caller has to make sure that:  - `this` points to a valid `PixelGrid` - the returned memory range is never accessed after the passed `PixelGrid` has been freed - the returned memory range is never accessed concurrently, either via the `PixelGrid` or directly</summary>
        [DllImport(__DllName, EntryPoint = "sp_pixel_grid_unsafe_data_ref", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern CByteSlice sp_pixel_grid_unsafe_data_ref(PixelGrid* @this);

        /// <summary>Turns a `Command` into a `Packet`. The `Command` gets consumed.  # Safety  The caller has to make sure that:  - `command` points to a valid instance of `Command` - `command` is not used concurrently or after this call - the returned `Packet` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_packet_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_packet_from_command", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* sp_packet_from_command(Command* command);

        /// <summary>Tries to load a `Packet` from the passed array with the specified length.  returns: NULL in case of an error, pointer to the allocated packet otherwise  # Safety  The caller has to make sure that:  - `data` points to a valid memory region of at least `length` bytes - `data` is not written to concurrently - the returned `Packet` instance is freed in some way, either by using a consuming function or by explicitly calling `sp_packet_dealloc`.</summary>
        [DllImport(__DllName, EntryPoint = "sp_packet_try_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* sp_packet_try_load(byte* data, nuint length);

        /// <summary>Deallocates a `Packet`.  # Safety  The caller has to make sure that:  - `this` points to a valid `Packet` - `this` is not used concurrently or after this call</summary>
        [DllImport(__DllName, EntryPoint = "sp_packet_dealloc", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void sp_packet_dealloc(Packet* @this);


    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct CBitVec
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct CBrightnessGrid
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct CCp437Grid
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct CByteSlice
    {
        public byte* start;
        public nuint length;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct Connection
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct PixelGrid
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct Packet
    {
    }


    public enum Command
    {
        Clear,
        Cp437Data,
        BitmapLinearWin,
        Brightness,
        CharBrightness,
        BitmapLinear,
        BitmapLinearAnd,
        BitmapLinearOr,
        BitmapLinearXor,
        HardReset,
        FadeOut,
        BitmapLegacy,
    }

    public enum CompressionCode : ushort
    {
        Uncompressed = 0,
        Zlib = 26490,
        Bzip2 = 25210,
        Lzma = 27770,
        Zstd = 31347,
    }


}
