using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Calculator.Interop;

namespace cs
{
	class Program
	{
		[STAThread]
		static void Main( string[] args )
		{
			// Rust COM object.
			ICalculator calc = new Calculator.Interop.Calculator();
			calc.Add(10);
			calc.Add(100);
			calc.Multiply(5);
			int result = calc.Substract(55);

			Console.WriteLine(result);
		}
	}
}
