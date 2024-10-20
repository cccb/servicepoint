// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace ServicePoint
{

    public unsafe sealed partial class Command: IDisposable
    {
#nullable enable
        /// <summary>
        ///  Tries to turn a [SPPacket] into a [SPCommand].
        ///
        ///  The packet is deallocated in the process.
        ///
        ///  Returns: pointer to new [SPCommand] instance or NULL if parsing failed.
        ///
        ///  # Panics
        ///
        ///  - when `packet` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - [SPPacket] points to a valid instance of [SPPacket]
        ///  - [SPPacket] is not used concurrently or after this call
        ///  - the result is checked for NULL
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: packet
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command? TryFromPacket(Packet packet)
        {
            var native = Command.sp_command_try_from_packet(packet.__Into());
            return native == null ? null : new Command(native);
        }

        /// <summary>
        ///  Clones a [SPCommand] instance.
        ///
        ///  returns: new [SPCommand] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `command` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `command` points to a valid instance of [SPCommand]
        ///  - `command` is not written to concurrently
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public Command Clone()
        {
            return new Command(Command.sp_command_clone(this.__Instance));
        }

        /// <summary>
        ///  Set all pixels to the off state.
        ///
        ///  Does not affect brightness.
        ///
        ///  Returns: a new [Command::Clear] instance. Will never return NULL.
        ///
        ///  # Examples
        ///
        ///  ```C
        ///  sp_connection_send_command(connection, sp_command_clear());
        ///  ```
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command Clear()
        {
            return new Command(Command.sp_command_clear());
        }

        /// <summary>
        ///  Kills the udp daemon on the display, which usually results in a restart.
        ///
        ///  Please do not send this in your normal program flow.
        ///
        ///  Returns: a new [Command::HardReset] instance. Will never return NULL.
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command HardReset()
        {
            return new Command(Command.sp_command_hard_reset());
        }

        /// <summary>
        ///  A yet-to-be-tested command.
        ///
        ///  Returns: a new `Command::FadeOut` instance. Will never return NULL.
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command FadeOut()
        {
            return new Command(Command.sp_command_fade_out());
        }

        /// <summary>
        ///  Set the brightness of all tiles to the same value.
        ///
        ///  Returns: a new [Command::Brightness] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - When the provided brightness value is out of range (0-11).
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command Brightness(byte brightness)
        {
            return new Command(Command.sp_command_brightness(brightness));
        }

        /// <summary>
        ///  Set the brightness of individual tiles in a rectangular area of the display.
        ///
        ///  The passed [SPBrightnessGrid] gets consumed.
        ///
        ///  Returns: a new [Command::CharBrightness] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `grid` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `grid` points to a valid instance of [SPBrightnessGrid]
        ///  - `grid` is not used concurrently or after this call
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: grid
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command CharBrightness(nuint x, nuint y, BrightnessGrid grid)
        {
            return new Command(Command.sp_command_char_brightness(x, y, grid.__Into()));
        }

        /// <summary>
        ///  Set pixel data starting at the pixel offset on screen.
        ///
        ///  The screen will continuously overwrite more pixel data without regarding the offset, meaning
        ///  once the starting row is full, overwriting will continue on column 0.
        ///
        ///  The contained [SPBitVec] is always uncompressed.
        ///
        ///  The passed [SPBitVec] gets consumed.
        ///
        ///  Returns: a new [Command::BitmapLinear] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `bit_vec` is null
        ///  - when `compression_code` is not a valid value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `bit_vec` points to a valid instance of [SPBitVec]
        ///  - `bit_vec` is not used concurrently or after this call
        ///  - `compression` matches one of the allowed enum values
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: bit_vec
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command BitmapLinear(nuint offset, BitVec bit_vec, CompressionCode compression)
        {
            return new Command(Command.sp_command_bitmap_linear(offset, bit_vec.__Into(), compression));
        }

        /// <summary>
        ///  Set pixel data according to an and-mask starting at the offset.
        ///
        ///  The screen will continuously overwrite more pixel data without regarding the offset, meaning
        ///  once the starting row is full, overwriting will continue on column 0.
        ///
        ///  The contained [SPBitVec] is always uncompressed.
        ///
        ///  The passed [SPBitVec] gets consumed.
        ///
        ///  Returns: a new [Command::BitmapLinearAnd] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `bit_vec` is null
        ///  - when `compression_code` is not a valid value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `bit_vec` points to a valid instance of [SPBitVec]
        ///  - `bit_vec` is not used concurrently or after this call
        ///  - `compression` matches one of the allowed enum values
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: bit_vec
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command BitmapLinearAnd(nuint offset, BitVec bit_vec, CompressionCode compression)
        {
            return new Command(Command.sp_command_bitmap_linear_and(offset, bit_vec.__Into(), compression));
        }

        /// <summary>
        ///  Set pixel data according to an or-mask starting at the offset.
        ///
        ///  The screen will continuously overwrite more pixel data without regarding the offset, meaning
        ///  once the starting row is full, overwriting will continue on column 0.
        ///
        ///  The contained [SPBitVec] is always uncompressed.
        ///
        ///  The passed [SPBitVec] gets consumed.
        ///
        ///  Returns: a new [Command::BitmapLinearOr] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `bit_vec` is null
        ///  - when `compression_code` is not a valid value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `bit_vec` points to a valid instance of [SPBitVec]
        ///  - `bit_vec` is not used concurrently or after this call
        ///  - `compression` matches one of the allowed enum values
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: bit_vec
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command BitmapLinearOr(nuint offset, BitVec bit_vec, CompressionCode compression)
        {
            return new Command(Command.sp_command_bitmap_linear_or(offset, bit_vec.__Into(), compression));
        }

        /// <summary>
        ///  Set pixel data according to a xor-mask starting at the offset.
        ///
        ///  The screen will continuously overwrite more pixel data without regarding the offset, meaning
        ///  once the starting row is full, overwriting will continue on column 0.
        ///
        ///  The contained [SPBitVec] is always uncompressed.
        ///
        ///  The passed [SPBitVec] gets consumed.
        ///
        ///  Returns: a new [Command::BitmapLinearXor] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `bit_vec` is null
        ///  - when `compression_code` is not a valid value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `bit_vec` points to a valid instance of [SPBitVec]
        ///  - `bit_vec` is not used concurrently or after this call
        ///  - `compression` matches one of the allowed enum values
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: bit_vec
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command BitmapLinearXor(nuint offset, BitVec bit_vec, CompressionCode compression)
        {
            return new Command(Command.sp_command_bitmap_linear_xor(offset, bit_vec.__Into(), compression));
        }

        /// <summary>
        ///  Show text on the screen.
        ///
        ///  The passed [SPCp437Grid] gets consumed.
        ///
        ///  Returns: a new [Command::Cp437Data] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `grid` is null
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `grid` points to a valid instance of [SPCp437Grid]
        ///  - `grid` is not used concurrently or after this call
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: grid
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command Cp437Data(nuint x, nuint y, Cp437Grid grid)
        {
            return new Command(Command.sp_command_cp437_data(x, y, grid.__Into()));
        }

        /// <summary>
        ///  Sets a window of pixels to the specified values.
        ///
        ///  The passed [SPBitmap] gets consumed.
        ///
        ///  Returns: a new [Command::BitmapLinearWin] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `bitmap` is null
        ///  - when `compression_code` is not a valid value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `bitmap` points to a valid instance of [SPBitmap]
        ///  - `bitmap` is not used concurrently or after this call
        ///  - `compression` matches one of the allowed enum values
        ///  - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_command_free`.
        ///
        ///  servicepoint_csbindgen_consumes: bitmap
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static Command BitmapLinearWin(nuint x, nuint y, Bitmap bitmap, CompressionCode compression_code)
        {
            return new Command(Command.sp_command_bitmap_linear_win(x, y, bitmap.__Into(), compression_code));
        }


#region internal machinery
        private SPCommand* _instance;
        internal SPCommand* __Instance
        {
            get
            {
                if (_instance == null)
                    throw new NullReferenceException("instance is null");
                return _instance;
            }
        }

        private Command(SPCommand* instance)
        {
            ArgumentNullException.ThrowIfNull(instance);
            _instance = instance;
        }

        internal SPCommand* __Into()
        {
            var instance = __Instance;
            _instance = null;
            return instance;
        }

        private void __Free()
        {
            if (_instance != null)
                Command.sp_command_free(__Into());
        }

        public void Dispose()
        {
            __Free();
            GC.SuppressFinalize(this);
        }

        ~Command() => __Free();
            
#endregion

#nullable restore
#region native methods
        const string __DllName = "servicepoint_binding_c";
        [DllImport(__DllName, EntryPoint = "sp_command_try_from_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_try_from_packet(SPPacket* packet);

        [DllImport(__DllName, EntryPoint = "sp_command_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_clone(SPCommand* command);

        [DllImport(__DllName, EntryPoint = "sp_command_clear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_clear();

        [DllImport(__DllName, EntryPoint = "sp_command_hard_reset", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_hard_reset();

        [DllImport(__DllName, EntryPoint = "sp_command_fade_out", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_fade_out();

        [DllImport(__DllName, EntryPoint = "sp_command_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_brightness(byte brightness);

        [DllImport(__DllName, EntryPoint = "sp_command_char_brightness", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_char_brightness(nuint x, nuint y, SPBrightnessGrid* grid);

        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_bitmap_linear(nuint offset, SPBitVec* bit_vec, CompressionCode compression);

        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_and", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_bitmap_linear_and(nuint offset, SPBitVec* bit_vec, CompressionCode compression);

        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_or", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_bitmap_linear_or(nuint offset, SPBitVec* bit_vec, CompressionCode compression);

        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_xor", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_bitmap_linear_xor(nuint offset, SPBitVec* bit_vec, CompressionCode compression);

        [DllImport(__DllName, EntryPoint = "sp_command_cp437_data", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_cp437_data(nuint x, nuint y, SPCp437Grid* grid);

        [DllImport(__DllName, EntryPoint = "sp_command_bitmap_linear_win", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPCommand* sp_command_bitmap_linear_win(nuint x, nuint y, SPBitmap* bitmap, CompressionCode compression_code);

        [DllImport(__DllName, EntryPoint = "sp_command_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern void sp_command_free(SPCommand* command);


#endregion
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct SPCommand
    {
    }



}
