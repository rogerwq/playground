using System;
using System.Runtime.InteropServices;

public class HelloWorld
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SampleStruct
    {
        public Int16 field_one;
        public Int32 field_two;
        public IntPtr string_field;
    }

    [DllImport("our_rust.dll")]
    private static extern Int32 add_numbers(Int32 number1, Int32 number2);
    [DllImport("our_rust.dll")]
    private static extern SampleStruct get_simple_struct();
    [DllImport("our_rust.dll")]
    private static extern SampleStruct free_string();
    
    public static void Main(string[] args)
    {
        var addedNumbers = add_numbers(10, 5);
        Console.WriteLine(addedNumbers);
        Console.WriteLine ("Hello Mono World");

        Console.WriteLine("\nTest Struct ...");
        var simple_struct = get_simple_struct();
        Console.WriteLine(simple_struct.field_one);
        Console.WriteLine(simple_struct.field_two);
        Console.WriteLine(Marshal.PtrToStringAnsi(simple_struct.string_field));
        free_string();
    }
}
