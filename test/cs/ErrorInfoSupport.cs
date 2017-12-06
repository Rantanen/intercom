using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
	[TestClass]
	public class ErrorInfoSupport
	{
		[TestMethod]
		public void ErrorsShowUpAsExceptions()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< COMException >(
						() => lib.StoreError( (int) 0x80004005, "E_FAIL error message" ),
						"E_FAIL error message" );
			}
		}

		[TestMethod]
		public void E_NOTIMPL_ConvertsToNotImplementedException()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< NotImplementedException >(
						() => lib.StoreError( (int) 0x80004001, "E_NOTIMPL error message" ),
						"E_NOTIMPL error message" );
			}
		}

		[TestMethod]
		public void E_INVALIDARG_ConvertsToInvalidArgumentException()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< ArgumentException >(
						() => lib.StoreError( (int) 0x80070057, "E_INVALIDARG error message" ),
						"E_INVALIDARG error message" );
			}
		}

		[TestMethod]
		public void E_POINTER_ConvertsToNullReferenceException()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< NullReferenceException >(
						() => lib.StoreError( (int) 0x80004003, "E_POINTER error message" ),
						"E_POINTER error message" );
			}
		}

		[TestMethod]
		public void E_NOINTERFACE_ConvertsToInvalidCastException()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< InvalidCastException >(
						() => lib.StoreError( (int) 0x80004002, "E_NOINTERFACE error message" ),
						"E_NOINTERFACE error message" );
			}
		}

		[TestMethod]
		public void E_ABORT_ConvertsToCOMException()
		{
			var lib = new TestLib.Interop.ErrorSource();
			unchecked
			{
				Assert.ThrowsException< COMException >(
						() => lib.StoreError( (int) 0x80004004, "E_ABORT error message" ),
						"E_ABORT error message" );
			}
		}
	}
}
