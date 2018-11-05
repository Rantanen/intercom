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
        public void VariantTypes()
        {
            var variantTest = new TestLib.VariantTests();
            Assert.IsTrue( variantTest.VariantParameter( 0, null ) );
            Assert.IsTrue( variantTest.VariantParameter( 2, (short) -1 ) );
            Assert.IsTrue( variantTest.VariantParameter( 3, -1 ) );
            Assert.IsTrue( variantTest.VariantParameter( 4, -1.0f ) );
            Assert.IsTrue( variantTest.VariantParameter( 5, -1.0d ) );
            Assert.IsTrue( variantTest.VariantParameter( 7, new DateTime( 0 ) ) );
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

        public class InterfaceClass : TestLib.IVariantInterface_Automation
        {
            public object DoStuff()
            {
                return 1;
            }
        }

        [TestMethod]
        public void InterfaceImplementation()
        {
            var variantTest = new TestLib.VariantTests();
            Assert.IsTrue( variantTest.VariantParameter( 13, new InterfaceClass() ) );
        }

        class DotNetClass
        {
            public object DoStuff()
            {
                return 1;
            }
        }

        [TestMethod]
        public void DotNetClassByParameter()
        {
            var variantTest = new TestLib.VariantTests();
            Assert.IsTrue( variantTest.VariantParameter( 13, new DotNetClass() ) );
        }

        struct DotNetStruct
        {
            public object DoStuff()
            {
                return 1;
            }
        }

        interface IDotNetInterface
        {
            int DoStuff();
        }

        class DotNetInterfaceImplementation : IDotNetInterface
        {
            public int DoStuff()
            {
                return 1;
            }
        }

        [TestMethod]
        public void DotNetInterfaceByParameter()
        {
            var variantTest = new TestLib.VariantTests();
            IDotNetInterface iinterface = new DotNetInterfaceImplementation();
            Assert.IsTrue( variantTest.VariantParameter( 13, iinterface ) );
        }
    }
}
