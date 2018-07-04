using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
    [TestClass]
    public class StatefulOperations
    {
        [TestMethod]
        public void StateIsStored()
        {
            var lib = new TestLib.Interop.StatefulOperations();

            var value = new Random().Next();
            lib.PutValue( value );
            Assert.AreEqual( value, lib.GetValue() );

            lib.PutValue( 0 );
            Assert.AreEqual( 0, lib.GetValue() );
        }
    }
}
