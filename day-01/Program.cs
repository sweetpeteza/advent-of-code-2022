using System.Threading.Channels;
using day_01;

Console.WriteLine("AOC Day 01");

string[] lines = File.ReadAllLines("./input.txt");

Console.WriteLine("Part 1");
Console.WriteLine("===========");
Console.WriteLine(PartOne.CorrectAnswer());
Console.WriteLine($"Max Elf: {PartOne.GetMaxElf(lines)}");

Console.WriteLine("===========");
Console.WriteLine($"Top 3 Elf Total:{PartTwo.GetElfTotals(lines).GetUpperXElfTotal(3)}");