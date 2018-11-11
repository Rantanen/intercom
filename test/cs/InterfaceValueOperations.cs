using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
    [TestClass]
    public class InterfaceValueOperations
    {
        [TestMethod]
        public void ComInterfaceAsReturnValue()
        {
            var creator = new TestLib.ClassCreator();
            var root = creator.CreateRoot( 1 );

            Assert.IsNotNull( root );
            Assert.AreEqual( 1, root.GetId() );
        }

        [TestMethod]
        public void ComInterfaceAsParameter()
        {
            var creator = new TestLib.ClassCreator();
            var root = creator.CreateRoot( 1 );

            Assert.IsNotNull( root );
            Assert.AreEqual( 1, root.GetId() );

            var child = creator.CreateChild( 2, ( TestLib.IParent_Automation ) root );

            Assert.AreEqual( 2, child.GetId() );
            Assert.AreEqual( 1, child.GetParentId() );
        }
    }
}
