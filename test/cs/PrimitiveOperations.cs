using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
    [TestClass]
    public class PrimitiveOperations
    {
        [TestMethod]
        public void SupportsInt64()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new Int64[] {0, 1, 10, Int64.MaxValue, Int64.MinValue})
            {
                Assert.AreEqual( (Int64)~( i + 1 ), lib.I64( i ) );
            }
        }

        [TestMethod]
        public void SupportsUInt64()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new UInt64[] {0, 1, 10, UInt64.MaxValue, UInt64.MinValue})
            {
                Assert.AreEqual( (UInt64)~( i + 1 ), lib.U64( i ) );
            }
        }

        [TestMethod]
        public void SupportsInt32()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new Int32[] {0, 1, 10, Int32.MaxValue, Int32.MinValue})
            {
                Assert.AreEqual( (Int32)~( i + 1 ), lib.I32( i ) );
            }
        }

        [TestMethod]
        public void SupportsUInt32()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new UInt32[] {0, 1, 10, UInt32.MaxValue, UInt32.MinValue})
            {
                Assert.AreEqual( (UInt32)~( i + 1 ), lib.U32( i ) );
            }
        }


        [TestMethod]
        public void SupportsInt16()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new Int16[] {0, 1, 10, Int16.MaxValue, Int16.MinValue})
            {
                Assert.AreEqual( (Int16)~( i + 1 ), lib.I16( i ) );
            }
        }

        [TestMethod]
        public void SupportsUInt16()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new UInt16[] {0, 1, 10, UInt16.MaxValue, UInt16.MinValue})
            {
                Assert.AreEqual( (UInt16)~( i + 1 ), lib.U16( i ) );
            }
        }


        [TestMethod]
        public void SupportsSByte()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new SByte[] {0, 1, 10, SByte.MaxValue, SByte.MinValue})
            {
                Assert.AreEqual( (SByte)~( i + 1 ), lib.I8( i ) );
            }
        }

        [TestMethod]
        public void SupportsByte()
        {
            var lib = new TestLib.PrimitiveOperations();
            foreach (var i in new Byte[] {0, 1, 10, Byte.MaxValue, Byte.MinValue})
            {
                Assert.AreEqual( (Byte)~( i + 1 ), lib.U8( i ) );
            }
        }
    }
}
