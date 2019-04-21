using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace Slashrev
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        const int c_unsetValue = 0;
        const string c_strRegPath = "Software\\MikeCal\\SlashRev";
        const string c_strRegMainX = "mainX";
        const string c_strRegMainY = "mainY";
        const string c_strRegMainWidth = "mainWidth";
        const string c_strRegMainHeight = "mainHeight";

        string output = string.Empty;

        public MainWindow()
        {
            InitializeComponent();
            LoadState();
            PopulateLabels();
        }

        protected override void OnKeyUp(KeyEventArgs e)
        {
            switch (e.Key)
            {
                case Key.Space:
                    PopulateLabels();
                    break;

                case Key.Enter:
                    Clipboard.SetText(this.output);
                    break;

                case Key.Escape:
                    this.Close();
                    break;
            }
            base.OnKeyUp(e);
        }

        protected override void OnClosing(System.ComponentModel.CancelEventArgs e)
        {
            SaveState();
        }

        void SaveState()
        {
            RegistryKey rk = Registry.CurrentUser.CreateSubKey(c_strRegPath);
            rk.SetValue(c_strRegMainX, this.Left, RegistryValueKind.DWord);
            rk.SetValue(c_strRegMainY, this.Top, RegistryValueKind.DWord);
            rk.SetValue(c_strRegMainWidth, this.Width, RegistryValueKind.DWord);
            rk.SetValue(c_strRegMainHeight, this.Height, RegistryValueKind.DWord);
        }

        void LoadState()
        {
            RegistryKey rk = Registry.CurrentUser.CreateSubKey(c_strRegPath);
            int x, y, width, height;

            x = (int)rk.GetValue(c_strRegMainX, c_unsetValue);
            y = (int)rk.GetValue(c_strRegMainY, c_unsetValue);
            width = (int)rk.GetValue(c_strRegMainWidth, c_unsetValue);
            height = (int)rk.GetValue(c_strRegMainHeight, c_unsetValue);

            if (x != c_unsetValue && y != c_unsetValue)
            {
                this.Left = x;
                this.Top = y;
            }

            if (width != c_unsetValue && height != c_unsetValue)
            {
                this.Width = width;
                this.Height = height;
            }
        }

        private void PopulateLabels()
        {
            string input = Clipboard.GetText();
            this.current.Content = input;
            this.output = ConvertSlashes(input);
            this.update.Content = this.output;
        }

        private string ConvertSlashes(string input)
        {
            const string c_slash = "/";
            const string c_backslash = "\\";

            string output = string.Empty;

            if (input.Contains(c_slash))
            {
                output = input.Replace(c_slash, c_backslash);
            }
            else if (input.Contains(c_backslash))
            {
                output = input.Replace(c_backslash, c_slash);
            }
            else
            {
                output = input;
            }

            return output;
        }
    }
}
