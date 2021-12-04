using System;
using System.IO;
 
namespace FindDupes
{
    class Program
    {
        static int totalNumDupes = 0;

        static void Main(string[] args)
        {
            string path = ".";
            string arg = "*.rpy";
            string[] files;

            if (args.Length == 1)
            {
                arg = args[0];
            }

            try
            {
                files = System.IO.Directory.GetFiles(path, arg);
                if (files.Length == 0)
                {
                    throw (new Exception());
                }
            }
            catch (Exception e)
            {
                Console.WriteLine("Couldn't find any files that match " + arg);
                Console.WriteLine("Error " + e.ToString());
                Usage();
                return;
            }

            HandleFiles(files);
            Console.WriteLine("Found " + totalNumDupes + " lines(s) with Dupes");
            Pause();
        }

        static void Pause()
        {
            Console.WriteLine("Press Enter to continue.");
            Console.ReadLine();
        }

        static void Usage()
        {
            Console.WriteLine("FindDupes by Tlaero");
            Console.WriteLine("Usage:");
            Console.WriteLine("  FindDupes <filename>");
            Console.WriteLine("To find the dupes in the file test.rpy");
            Console.WriteLine("  FindDupes test.rpy");
            Console.WriteLine("To find the dupes in all rpy files");
            Console.WriteLine("  FindDupes *.rpy");
            Pause();
        }

        static void HandleFiles(string[] files)
        {
            foreach (string file in files)
            {
                //Console.WriteLine("Scanning file: " + file);
                string[] lines;
                try
                {
                    StreamReader sr = new StreamReader(file);
                    string str = sr.ReadToEnd();
                    sr.Close();

                    lines = str.Replace("\r", "").Split(new char[] { '\n' });
                    ScanFile(file, lines);
                }
                catch
                {
                    Console.WriteLine("Error opening file: " + file);
                    continue;
                }


            }
        }

        static void ScanFile(string file, string[] lines)
        {
            bool anyDupesFound = false;

            foreach (string line in lines)
            {
                string trimmed = line.Trim();
                string[] words = trimmed.Split(new char[] { ' ' });
                bool hasDupe = HasDupes(words);
                if (hasDupe)
                {
                    if (!anyDupesFound)
                    {
                        Console.WriteLine("Dupes found in file: " + file);
                    }

                    Console.WriteLine("Line: " + line);
                    anyDupesFound = true;
                    totalNumDupes++;
                }
            }
        }

        static bool HasDupes(string[] words)
        {
            if (words[0] != "show")
            {
                return false;
            }

            for (int i = 1; i < words.Length - 1; i++)
            {
                if (words[i].Trim() == string.Empty)
                {
                    continue;
                }

                for (int j = i + 1; j < words.Length; j++)
                {
                    if (words[i].ToLower() == words[j].ToLower())
                    {
                        return true;
                    }
                }
            }
            return false;
        }
    }
}
