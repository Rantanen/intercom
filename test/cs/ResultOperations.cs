using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
	[TestClass]
	public class ResultOperations
	{
		[TestMethod]
		public void NotImplemented()
		{
			var lib = new TestLib.Interop.ResultOperations();
			Assert.ThrowsException< NotImplementedException >( () => lib.NotImpl() );
		}

		[TestMethod]
		public void ArgumentException()
		{
			var lib = new TestLib.Interop.ResultOperations();
			Assert.ThrowsException< ArgumentException >( () => lib.Sqrt( -1 ) );

			var value = new Random().NextDouble();
			Assert.AreEqual( Math.Sqrt( value ), lib.Sqrt( value ) );
		}

		[TestMethod]
		public void Success()
		{
			var lib = new TestLib.Interop.ResultOperations();
			lib.SOk();
		}

		[TestMethod]
		public void Tuples()
		{
			var lib = new TestLib.Interop.ResultOperations();

			ushort left;
			ushort right;
			lib.Tuple( 0xCAFEBABE, out left, out right );

			Assert.AreEqual( 0xCAFE, left );
			Assert.AreEqual( 0xBABE, right );
		}
	}
}
