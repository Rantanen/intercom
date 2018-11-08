using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cs
{
    [TestClass]
    public class VariantTests
    {
        [TestMethod]
        public void VariantsAsComToRustParameters()
        {
            var variantTest = new TestLib.VariantTests();
            Assert.IsTrue( variantTest.VariantParameter( 0, null ) );
            Assert.IsTrue( variantTest.VariantParameter( 2, (short) -1 ) );
            Assert.IsTrue( variantTest.VariantParameter( 3, -1 ) );
            Assert.IsTrue( variantTest.VariantParameter( 4, -1.0f ) );
            Assert.IsTrue( variantTest.VariantParameter( 5, -1.0d ) );
            Assert.IsTrue( variantTest.VariantParameter( 701,
                    DateTime.Parse( "1899-12-30T00:00:00" ) ) );
            Assert.IsTrue( variantTest.VariantParameter( 702,
                    DateTime.Parse( "2000-01-02T03:04:05" ) ) );
            Assert.IsTrue( variantTest.VariantParameter( 703,
                    DateTime.Parse( "2000-01-01T00:00:00" ) ) );
            Assert.IsTrue( variantTest.VariantParameter( 704,
                    DateTime.Parse( "1800-01-02T03:04:05" ) ) );
            Assert.IsTrue( variantTest.VariantParameter( 705,
                    DateTime.Parse( "1800-01-01T00:00:00" ) ) );
            Assert.IsTrue( variantTest.VariantParameter( 8, "text" ) );
            Assert.IsTrue( variantTest.VariantParameter( 9, new object() ) );
            Assert.IsTrue( variantTest.VariantParameter( 11, true ) );
            Assert.IsTrue( variantTest.VariantParameter( 14, 1.0m ) );
            Assert.IsTrue( variantTest.VariantParameter( 16, (sbyte) -1 ) );
            Assert.IsTrue( variantTest.VariantParameter( 17, (byte) 129 ) );
            Assert.IsTrue( variantTest.VariantParameter( 18, (ushort) 12929 ) );
            Assert.IsTrue( variantTest.VariantParameter( 19, 1292929u ) );
            Assert.IsTrue( variantTest.VariantParameter( 20, -1L ) );
            Assert.IsTrue( variantTest.VariantParameter( 21, 129292929UL ) );
        }

        [TestMethod]
        public void VariantsAsRustToComParameters()
        {
            var variantTest = new TestLib.VariantTests();
            Assert.AreEqual( null, variantTest.VariantResult( 0 ) );
            Assert.AreEqual( ( short ) -1, variantTest.VariantResult( 2 ) );
            Assert.AreEqual( -1, variantTest.VariantResult( 3 ) );
            Assert.AreEqual( -1.0f, variantTest.VariantResult( 4 ) );
            Assert.AreEqual( -1.0d, variantTest.VariantResult( 5 ) );
            Assert.AreEqual( DateTime.Parse( "1899-12-30T00:00:00" ),
                    variantTest.VariantResult( 701 ) );
            Assert.AreEqual( DateTime.Parse( "2000-01-02T03:04:05" ),
                    variantTest.VariantResult( 702 ) );
            Assert.AreEqual( DateTime.Parse( "2000-01-01T00:00:00" ),
                    variantTest.VariantResult( 703 ) );
            Assert.AreEqual( DateTime.Parse( "1800-01-02T03:04:05" ),
                    variantTest.VariantResult( 704 ) );
            Assert.AreEqual( DateTime.Parse( "1800-01-01T00:00:00" ),
                    variantTest.VariantResult( 705 ) );
            Assert.AreEqual( "text", variantTest.VariantResult( 801 ) );  // BString
            Assert.AreEqual( "text", variantTest.VariantResult( 802 ) );  // String
            Assert.AreEqual( "text", variantTest.VariantResult( 803 ) );  // CString
            Assert.AreEqual( true, variantTest.VariantResult( 11 ) );
            Assert.AreEqual( (sbyte) -1, variantTest.VariantResult( 16 ) );
            Assert.AreEqual( (byte) 129, variantTest.VariantResult( 17 ) );
            Assert.AreEqual( (ushort) 12929, variantTest.VariantResult( 18 ) );
            Assert.AreEqual( 1292929u, variantTest.VariantResult( 19 ) );
            Assert.AreEqual( -1L, variantTest.VariantResult( 20 ) );
            Assert.AreEqual( 129292929UL, variantTest.VariantResult( 21 ) );
        }

        public class InterfaceClass : TestLib.IVariantInterface_Automation
        {
            public object DoStuff()
            {
                return nameof( InterfaceClass );
            }
        }

        [TestMethod]
        public void InterfaceImplementation()
        {
            var variantTest = new TestLib.VariantTests();
            dynamic d = variantTest.VariantInterface( new InterfaceClass() );
            Assert.AreEqual( nameof( InterfaceClass ), d );
        }

        class DotNetClass
        {
            public object DoStuff()
            {
                return nameof( DotNetClass );
            }
        }

        [TestMethod, Ignore( "Requires IDispatch support")]
        public void DotNetClassByParameter()
        {
            var variantTest = new TestLib.VariantTests();
            dynamic d = variantTest.VariantInterface( new DotNetClass() );
            Assert.AreEqual( nameof( DotNetClass ), d );
        }

        struct DotNetStruct
        {
            public object DoStuff()
            {
                return nameof( DotNetStruct );
            }
        }

        interface IDotNetInterface
        {
            string DoStuff();
        }

        class DotNetInterfaceImplementation : IDotNetInterface
        {
            public string DoStuff()
            {
                return nameof( DotNetInterfaceImplementation );
            }
        }

        [TestMethod, Ignore( "Requires IDispatch support")]
        public void DotNetInterfaceByParameter()
        {
            var variantTest = new TestLib.VariantTests();
            IDotNetInterface iinterface = new DotNetInterfaceImplementation();
            dynamic d = variantTest.VariantInterface( iinterface );
            Assert.AreEqual( nameof( DotNetInterfaceImplementation ), d );
        }
    }
}
