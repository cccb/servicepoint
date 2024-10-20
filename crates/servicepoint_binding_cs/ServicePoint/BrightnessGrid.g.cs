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

    public unsafe sealed partial class BrightnessGrid: IDisposable
    {
#nullable enable
        /// <summary>
        ///  Creates a new [SPBrightnessGrid] with the specified dimensions.
        ///
        ///  returns: [SPBrightnessGrid] initialized to 0. Will never return NULL.
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - the returned instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_brightness_grid_free`.
        /// </summary>
        public BrightnessGrid(nuint width, nuint height) : this(sp_brightness_grid_new(width, height)) {}

        /// <summary>
        ///  Loads a [SPBrightnessGrid] with the specified dimensions from the provided data.
        ///
        ///  returns: new [SPBrightnessGrid] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `data` is NULL
        ///  - when the provided `data_length` does not match `height` and `width`
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `data` points to a valid memory location of at least `data_length`
        ///    bytes in size.
        ///  - the returned instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_brightness_grid_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public static BrightnessGrid Load(nuint width, nuint height, byte* data, nuint data_length)
        {
            return new BrightnessGrid(BrightnessGrid.sp_brightness_grid_load(width, height, data, data_length));
        }

        /// <summary>
        ///  Clones a [SPBrightnessGrid].
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to read from
        ///
        ///  returns: new [SPBrightnessGrid] instance. Will never return NULL.
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        ///  - `brightness_grid` is not written to concurrently
        ///  - the returned instance is freed in some way, either by using a consuming function or
        ///    by explicitly calling `sp_brightness_grid_free`.
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public BrightnessGrid Clone()
        {
            return new BrightnessGrid(BrightnessGrid.sp_brightness_grid_clone(this.__Instance));
        }

        /// <summary>
        ///  Gets the current value at the specified position.
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to read from
        ///  - `x` and `y`: position of the cell to read
        ///
        ///  returns: value at position
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///  - When accessing `x` or `y` out of bounds.
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        ///  - `brightness_grid` is not written to concurrently
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public byte Get(nuint x, nuint y)
        {
            return BrightnessGrid.sp_brightness_grid_get(this.__Instance, x, y);
        }

        /// <summary>
        ///  Sets the value of the specified position in the [SPBrightnessGrid].
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to write to
        ///  - `x` and `y`: position of the cell
        ///  - `value`: the value to write to the cell
        ///
        ///  returns: old value of the cell
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///  - When accessing `x` or `y` out of bounds.
        ///  - When providing an invalid brightness value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBitVec]
        ///  - `brightness_grid` is not written to or read from concurrently
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public void Set(nuint x, nuint y, byte value)
        {
            BrightnessGrid.sp_brightness_grid_set(this.__Instance, x, y, value);
        }

        /// <summary>
        ///  Sets the value of all cells in the [SPBrightnessGrid].
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to write to
        ///  - `value`: the value to set all cells to
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///  - When providing an invalid brightness value
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        ///  - `brightness_grid` is not written to or read from concurrently
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public void Fill(byte value)
        {
            BrightnessGrid.sp_brightness_grid_fill(this.__Instance, value);
        }

        /// <summary>
        ///  Gets the width of the [SPBrightnessGrid] instance.
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to read from
        ///
        ///  returns: width
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public nuint Width()
        {
            return BrightnessGrid.sp_brightness_grid_width(this.__Instance);
        }

        /// <summary>
        ///  Gets the height of the [SPBrightnessGrid] instance.
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to read from
        ///
        ///  returns: height
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public nuint Height()
        {
            return BrightnessGrid.sp_brightness_grid_height(this.__Instance);
        }

        /// <summary>
        ///  Gets an unsafe reference to the data of the [SPBrightnessGrid] instance.
        ///
        ///  # Arguments
        ///
        ///  - `brightness_grid`: instance to read from
        ///
        ///  returns: slice of bytes underlying the `brightness_grid`.
        ///
        ///  # Panics
        ///
        ///  - when `brightness_grid` is NULL
        ///
        ///  # Safety
        ///
        ///  The caller has to make sure that:
        ///
        ///  - `brightness_grid` points to a valid [SPBrightnessGrid]
        ///  - the returned memory range is never accessed after the passed [SPBrightnessGrid] has been freed
        ///  - the returned memory range is never accessed concurrently, either via the [SPBrightnessGrid] or directly
        /// </summary>
        [System.Runtime.CompilerServices.MethodImplAttribute(System.Runtime.CompilerServices.MethodImplOptions.AggressiveInlining)]
        public SPByteSlice UnsafeDataRef()
        {
            return BrightnessGrid.sp_brightness_grid_unsafe_data_ref(this.__Instance);
        }


#region internal machinery
        private SPBrightnessGrid* _instance;
        internal SPBrightnessGrid* __Instance
        {
            get
            {
                if (_instance == null)
                    throw new NullReferenceException("instance is null");
                return _instance;
            }
        }

        private BrightnessGrid(SPBrightnessGrid* instance)
        {
            ArgumentNullException.ThrowIfNull(instance);
            _instance = instance;
        }

        internal SPBrightnessGrid* __Into()
        {
            var instance = __Instance;
            _instance = null;
            return instance;
        }

        private void __Free()
        {
            if (_instance != null)
                BrightnessGrid.sp_brightness_grid_free(__Into());
        }

        public void Dispose()
        {
            __Free();
            GC.SuppressFinalize(this);
        }

        ~BrightnessGrid() => __Free();
            
#endregion

#nullable restore
#region native methods
        const string __DllName = "servicepoint_binding_c";
        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_new", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPBrightnessGrid* sp_brightness_grid_new(nuint width, nuint height);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_load", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPBrightnessGrid* sp_brightness_grid_load(nuint width, nuint height, byte* data, nuint data_length);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_clone", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPBrightnessGrid* sp_brightness_grid_clone(SPBrightnessGrid* brightness_grid);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_free", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern void sp_brightness_grid_free(SPBrightnessGrid* brightness_grid);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_get", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern byte sp_brightness_grid_get(SPBrightnessGrid* brightness_grid, nuint x, nuint y);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_set", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern void sp_brightness_grid_set(SPBrightnessGrid* brightness_grid, nuint x, nuint y, byte value);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_fill", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern void sp_brightness_grid_fill(SPBrightnessGrid* brightness_grid, byte value);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_width", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern nuint sp_brightness_grid_width(SPBrightnessGrid* brightness_grid);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_height", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern nuint sp_brightness_grid_height(SPBrightnessGrid* brightness_grid);

        [DllImport(__DllName, EntryPoint = "sp_brightness_grid_unsafe_data_ref", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        private static extern SPByteSlice sp_brightness_grid_unsafe_data_ref(SPBrightnessGrid* brightness_grid);


#endregion
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe partial struct SPBrightnessGrid
    {
    }



}
