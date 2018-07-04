using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Security;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
    [TestClass]
    public class SetUpFixture
    {
        private static IntPtr hActCtx;
        private static IntPtr cookie;

        [AssemblyInitialize]
        public static void SetUp( TestContext testContext )
        {
            UnsafeNativeMethods.ACTCTX context = new UnsafeNativeMethods.ACTCTX();
            context.cbSize = Marshal.SizeOf( typeof( UnsafeNativeMethods.ACTCTX ) );
            var manifest = Path.Combine( Path.GetDirectoryName( Assembly.GetExecutingAssembly().Location ), "test_lib.dll" );
            context.dwFlags = 0x008;
            context.lpSource = manifest;
            context.lpResourceName = new IntPtr( 1 );

            hActCtx = UnsafeNativeMethods.CreateActCtx( ref context );
            if( hActCtx == ( IntPtr ) ( -1 ) )
                throw new Win32Exception( Marshal.GetLastWin32Error() );

            cookie = IntPtr.Zero;
            if( !UnsafeNativeMethods.ActivateActCtx( hActCtx, out cookie ) )
                throw new Win32Exception( Marshal.GetLastWin32Error() );
        }

        [AssemblyCleanup]
        public static void TearDown()
        {
            UnsafeNativeMethods.DeactivateActCtx( 0, cookie );
            UnsafeNativeMethods.ReleaseActCtx( hActCtx );
        }
    }

    [SuppressUnmanagedCodeSecurity]
    internal static class UnsafeNativeMethods
    {
        // Activation Context API Functions
        [DllImport("Kernel32.dll", SetLastError = true, EntryPoint = "CreateActCtxW")]
        internal extern static IntPtr CreateActCtx(ref ACTCTX actctx);

        [DllImport("Kernel32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        internal static extern bool ActivateActCtx(IntPtr hActCtx, out IntPtr lpCookie);

        [DllImport("kernel32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        internal static extern bool DeactivateActCtx(int dwFlags, IntPtr lpCookie);

        [DllImport("Kernel32.dll", SetLastError = true)]
        internal static extern void ReleaseActCtx(IntPtr hActCtx);

        // Activation context structure
        [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
        internal struct ACTCTX
        {
            public int cbSize;
            public uint dwFlags;
            public string lpSource;
            public ushort wProcessorArchitecture;
            public Int16 wLangId;
            public string lpAssemblyDirectory;
            public IntPtr lpResourceName;
            public string lpApplicationName;
            public IntPtr hModule;
        }

    }
}
