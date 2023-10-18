using System.Diagnostics;
using System.Threading.Channels;
using day_01;

var sw = Stopwatch.StartNew();

Console.WriteLine("AOC Day 01");

string[] lines = File.ReadAllLines("./input.txt");

//Console.WriteLine("Part 1");
//Console.WriteLine("===========");
//Console.WriteLine(PartOne.CorrectAnswer());
Console.WriteLine("===========");
Console.WriteLine($"Max Elf: {PartOne.GetMaxElf(lines)}");
sw.Stop();

var sw2 = Stopwatch.StartNew();

Console.WriteLine("===========");
Console.WriteLine($"Completed in {sw.ElapsedMilliseconds}ms");

Console.WriteLine("===========");
Console.WriteLine($"Top 3 Elf Total:{PartTwo.GetElfTotals(lines).GetUpperXElfTotal(3)}");

sw2.Stop();

Console.WriteLine("===========");
Console.WriteLine($"Completed in {sw.ElapsedMilliseconds}ms");



