use std::time::Instant;
use std::thread;

pub fn run_fibonacci(){
    let n = 40; // Change for a different test case
    let start = Instant::now();

    let result = fibonacci(n);

    let duration = start.elapsed();
    println!("Rust: Fibonacci({}) = {}", n, result);
    println!("Execution time: {:?}", duration);
}

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        n as u64
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn sum_of_large_array(){
    let size = 100_000_000;
    let mut numbers: Vec<i32> = (0..size).collect();

    let start = Instant::now();
    let sum: i64 = numbers.iter().map(|&x| x as i64).sum();
    let duration = start.elapsed();

    println!("Rust: Sum = {}", sum);
    println!("Execution time: {:?}", duration);    
}

fn expensive_computation(n: u32) -> u64 {
    (1..=n as u64).product()
}

pub fn multi_threading(){
    let start = Instant::now();

    let handles: Vec<_> = (1..=4)
        .map(|i| thread::spawn(move || expensive_computation(20 + i)))
        .collect();

    for handle in handles {
        println!("Result: {}", handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("Rust: Execution time: {:?}", duration);    
}

/* below equivalent in c#
---------------------------
using System;
using System.Diagnostics;

class Program
{
    static ulong Fibonacci(int n)
    {
        if (n <= 1) return (ulong)n;
        return Fibonacci(n - 1) + Fibonacci(n - 2);
    }

    static void Main()
    {
        int n = 40; // Change for a different test case
        Stopwatch sw = Stopwatch.StartNew();

        ulong result = Fibonacci(n);

        sw.Stop();
        Console.WriteLine($"C#: Fibonacci({n}) = {result}");
        Console.WriteLine($"Execution time: {sw.Elapsed}");
    }
}
---------------------------
using System;
using System.Diagnostics;
using System.Linq;

class Program
{
    static void Main()
    {
        int size = 100_000_000;
        int[] numbers = Enumerable.Range(0, size).ToArray();
        
        Stopwatch sw = Stopwatch.StartNew();
        long sum = numbers.Sum(x => (long)x);
        sw.Stop();

        Console.WriteLine($"C#: Sum = {sum}");
        Console.WriteLine($"Execution time: {sw.Elapsed}");
    }
}
---------------------------
using System;
using System.Diagnostics;
using System.Threading.Tasks;

class Program
{
    static ulong ExpensiveComputation(int n)
    {
        ulong result = 1;
        for (int i = 1; i <= n; i++) result *= (ulong)i;
        return result;
    }

    static void Main()
    {
        Stopwatch sw = Stopwatch.StartNew();
        
        Task<ulong>[] tasks = new Task<ulong>[4];
        for (int i = 0; i < 4; i++)
        {
            int val = 20 + i;
            tasks[i] = Task.Run(() => ExpensiveComputation(val));
        }

        Task.WaitAll(tasks);

        foreach (var task in tasks)
            Console.WriteLine($"Result: {task.Result}");

        sw.Stop();
        Console.WriteLine($"C#: Execution time: {sw.Elapsed}");
    }
}

 */