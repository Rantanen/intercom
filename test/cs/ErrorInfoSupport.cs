using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace cs
{
    public class ErrorSource : TestLib.IErrorSource_Automation
    {
        public class CustomException : Exception
        {
            public CustomException( int hr, string msg )
                : base( msg )
            {
                this.HResult = hr;
            }
        }

        public void ReturnComerror( int hr, string desc )
        {
            throw new CustomException( hr, desc );
        }

        public void ReturnTesterror( int hr, string desc )
        {
            throw new CustomException( hr, desc );
        }

        public void ReturnIoerror( int hr, string desc )
        {
            throw new CustomException( hr, desc );
        }
    }

    [TestClass]
    public class ErrorInfoSupport
    {
        [TestMethod]
        public void ErrorsShowUpAsExceptions()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< COMException >(
                        () => lib.ReturnComerror( (int) 0x80004005, "E_FAIL error message" ) );
                Assert.AreEqual( "E_FAIL error message", ex.Message );
                ex = Assert.ThrowsException< COMException >(
                        () => lib.ReturnTesterror( (int) 0x80004005, "E_FAIL error message" ) );
                Assert.AreEqual( "E_FAIL error message", ex.Message );
            }
        }

        [TestMethod]
        public void E_NOTIMPL_ConvertsToNotImplementedException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< NotImplementedException >(
                        () => lib.ReturnComerror( (int) 0x80004001, "E_NOTIMPL error message" ) );
                Assert.AreEqual( "E_NOTIMPL error message", ex.Message );
                ex = Assert.ThrowsException< NotImplementedException >(
                        () => lib.ReturnTesterror( (int) 0x80004001, "E_NOTIMPL error message" ) );
                Assert.AreEqual( "E_NOTIMPL error message", ex.Message );
            }
        }

        [TestMethod]
        public void E_INVALIDARG_ConvertsToInvalidArgumentException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< ArgumentException >(
                        () => lib.ReturnComerror( (int) 0x80070057, "E_INVALIDARG error message" ) );
                Assert.AreEqual( "E_INVALIDARG error message", ex.Message );
                ex = Assert.ThrowsException< ArgumentException >(
                        () => lib.ReturnTesterror( (int) 0x80070057, "E_INVALIDARG error message" ) );
                Assert.AreEqual( "E_INVALIDARG error message", ex.Message );
            }
        }

        [TestMethod]
        public void E_POINTER_ConvertsToNullReferenceException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< NullReferenceException >(
                        () => lib.ReturnComerror( (int) 0x80004003, "E_POINTER error message" ) );
                Assert.AreEqual( "E_POINTER error message", ex.Message );
                ex = Assert.ThrowsException< NullReferenceException >(
                        () => lib.ReturnTesterror( (int) 0x80004003, "E_POINTER error message" ) );
                Assert.AreEqual( "E_POINTER error message", ex.Message );
            }
        }

        [TestMethod]
        public void E_NOINTERFACE_ConvertsToInvalidCastException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< InvalidCastException >(
                        () => lib.ReturnComerror( (int) 0x80004002, "E_NOINTERFACE error message" ) );
                Assert.AreEqual( "E_NOINTERFACE error message", ex.Message );
                ex = Assert.ThrowsException< InvalidCastException >(
                        () => lib.ReturnTesterror( (int) 0x80004002, "E_NOINTERFACE error message" ) );
                Assert.AreEqual( "E_NOINTERFACE error message", ex.Message );
            }
        }

        [TestMethod]
        public void E_ABORT_ConvertsToCOMException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            unchecked
            {
                var ex = Assert.ThrowsException< COMException >(
                        () => lib.ReturnComerror( (int) 0x80004004, "E_ABORT error message" ) );
                Assert.AreEqual( "E_ABORT error message", ex.Message );
                ex = Assert.ThrowsException< COMException >(
                        () => lib.ReturnTesterror( (int) 0x80004004, "E_ABORT error message" ) );
                Assert.AreEqual( "E_ABORT error message", ex.Message );
            }
        }

        [TestMethod]
        public void AccessDenied_ConvertsToUnauthorizedAccessException()
        {
            var lib = ( TestLib.IErrorSource_Automation )new TestLib.ErrorTests();
            var ex = Assert.ThrowsException< UnauthorizedAccessException >(
                    () => lib.ReturnIoerror( 5, "" ) );
            Assert.AreEqual( "permission denied", ex.Message );
        }

        [TestMethod]
        public void CallbackComErrorMaintainsErrorInfo()
        {
            var lib = new TestLib.ErrorTests();
            lib.TestComerror( new ErrorSource() );
        }

        [TestMethod]
        public void CallbackTestErrorMaintainsErrorInfo()
        {
            var lib = new TestLib.ErrorTests();
            lib.TestTesterror( new ErrorSource() );
        }

        [TestMethod]
        public void CallbackIoErrorMaintainsErrorInfo()
        {
            var lib = new TestLib.ErrorTests();
            lib.TestIoerror( new ErrorSource() );
        }
    }
}
