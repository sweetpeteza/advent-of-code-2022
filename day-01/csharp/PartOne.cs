namespace day_01;

public class PartOne
{
    public static int MaxNormal(string[] lines)
    {
        List<int> elves = new();

        int currentElf = 0;

        foreach (string line in lines)
        {
            if (string.IsNullOrEmpty(line))
            {
                elves.Add(currentElf);
                currentElf = 0;
            }
            else
            {
                currentElf += int.Parse(line);
            }
        }

        int maxElf = elves
            .OrderByDescending(x => x)
            .FirstOrDefault();

        return maxElf;
    }

    public static double GetMaxElf(IEnumerable<string> lines)
    {
        double maxValue = 0;
        double currentValue = 0;
        foreach (string line in lines)
        {
            if (string.IsNullOrEmpty(line))
            {
                maxValue = currentValue > maxValue ? currentValue : maxValue;
                currentValue = 0;
            }
            else
            {
                currentValue += double.TryParse(line, out double val) ? val : 0;
            }
        }

        return maxValue;
    }
    
    public static double GetMaxElfV2(IEnumerable<string> lines)
    {
        return PartTwo.GetElfTotals(lines).Max();
    }
    
    public static int CorrectAnswer() => 77501;
}

public static class PartTwo
{
    public static IEnumerable<double> GetElfTotals(IEnumerable<string> lines)
    {
        double currentValue = 0;
        foreach (string line in lines)
        {
            if (string.IsNullOrEmpty(line))
            {
                yield return currentValue;
                currentValue = 0;
            }
            else
            {
                currentValue += double.TryParse(line, out double val) ? val : 0;
            }
        }

        yield return currentValue;
    }

    public static double GetUpperXElfTotal(this IEnumerable<double> elfTotals, int number)
        => elfTotals.OrderByDescending(x => x)
            .Take(3)
            .Sum();
}