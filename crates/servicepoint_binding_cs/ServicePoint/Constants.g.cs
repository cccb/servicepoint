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
    public static unsafe partial class ConstantsNative
    {
        const string __DllName = "servicepoint_binding_c";

        public const nuint SP_TILE_SIZE = 8;
        public const nuint SP_TILE_WIDTH = 56;
        public const nuint SP_TILE_HEIGHT = 20;
        public const byte SP_BRIGHTNESS_MIN = 0;
        public const byte SP_BRIGHTNESS_MAX = 11;
        public const byte SP_BRIGHTNESS_LEVELS = 12;



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
